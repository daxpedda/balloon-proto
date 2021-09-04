use std::os::raw::{c_char, c_int};

// Maximum password blob length (in bytes).
pub const BLOB_LEN: usize = 160;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BalloonOptions {
	pub s_cost: u32,
	pub t_cost: u32,
	pub n_threads: u32,
}

extern "C" {
	#[link_name = "Balloon_Hash"]
	pub fn hash(
		out: *mut c_char,
		opt: *mut BalloonOptions,
		passwd: *const c_char,
		passwd_len: usize,
	) -> c_int;

	#[link_name = "Balloon_Verify"]
	pub fn verify(blob: *const c_char, passwd: *const c_char, passwd_len: usize) -> c_int;
}
