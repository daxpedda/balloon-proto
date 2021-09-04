use std::{os::raw::c_int, string::FromUtf8Error};

use thiserror::Error;

mod balloon;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
	#[error(transparent)]
	Utf8(#[from] FromUtf8Error),
	#[error("`blob` can't be longer then {}", balloon::BLOB_LEN)]
	BlobTooLong,
	#[error("hash mismatch")]
	HashMismatch,
	#[error("unknown error code: {0}")]
	Unknown(i64),
}

impl Error {
	fn from_code(code: c_int) -> Result<()> {
		match code {
			0 => Ok(()),
			16 => Err(Self::HashMismatch),
			code => Err(Self::Unknown(code.into())),
		}
	}
}

pub fn hash(passwd: &[u8], s_cost: u32, t_cost: u32, n_threads: u32) -> Result<String> {
	let mut out = [0_u8; balloon::BLOB_LEN];
	let mut opt = balloon::BalloonOptions {
		s_cost,
		t_cost,
		n_threads,
	};

	let result = unsafe {
		balloon::hash(
			out.as_mut_ptr().cast(),
			&mut opt,
			passwd.as_ptr().cast(),
			passwd.len(),
		)
	};

	Error::from_code(result)?;
	Ok(String::from_utf8(out.to_vec())?)
}

pub fn verify(blob: &str, passwd: &[u8]) -> Result<()> {
	let result = if blob.len() > balloon::BLOB_LEN {
		return Err(Error::BlobTooLong);
	} else if blob.len() == balloon::BLOB_LEN {
		unsafe { balloon::verify(blob.as_ptr().cast(), passwd.as_ptr().cast(), passwd.len()) }
	} else {
		let mut blob_new = [0_u8; balloon::BLOB_LEN];
		blob_new[..blob.len()].copy_from_slice(blob.as_bytes());
		unsafe {
			balloon::verify(
				blob_new.as_ptr().cast(),
				passwd.as_ptr().cast(),
				passwd.len(),
			)
		}
	};

	Error::from_code(result)
}

#[test]
fn success() -> Result<()> {
	let phc = hash(b"test", 1024, 1, 1)?;
	verify(&phc, b"test")
}

#[test]
fn failure() -> Result<()> {
	let phc = hash(b"test1", 1024, 1, 1)?;
	assert_eq!(Err(Error::HashMismatch), verify(&phc, b"test2"));
	Ok(())
}

#[test]
fn blob_len() -> anyhow::Result<()> {
	assert_eq!(
		Err(Error::BlobTooLong),
		verify(std::str::from_utf8(&[0; balloon::BLOB_LEN + 1])?, &[])
	);

	Ok(())
}
