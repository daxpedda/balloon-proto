use std::{env, path::PathBuf, process::Command};

use anyhow::Result;

fn main() -> Result<()> {
	if cfg!(not(target_os = "linux")) {
		panic!("Only Linux is supported")
	}

	// determine path to build OpenSSL in
	let mut openssl_path = if let Some(openssl_path) = env::var_os("BALLOON_OPENSSL_INSTALL") {
		PathBuf::from(openssl_path)
	} else {
		PathBuf::from(env::var("OUT_DIR")?)
	};
	openssl_path.push("openssl");

	// get newest 1.0 version of OpenSSL
	if !openssl_path.join("include").exists()
		|| !openssl_path.join("libssl.so").exists()
		|| !openssl_path.join("libcrypto.so").exists()
	{
		// download source
		assert!(Command::new("wget")
			.args(&[
				"https://www.openssl.org/source/old/1.0.2/openssl-1.0.2u.tar.gz",
				"-nc",
				"-P",
			])
			.arg(&openssl_path)
			.status()?
			.success());
		// extract
		assert!(Command::new("tar")
			.current_dir(&openssl_path)
			.args(&["-xzf", "openssl-1.0.2u.tar.gz", "--strip-components", "1"])
			.status()
			.expect("failed to execute process")
			.success());
		// configure
		assert!(Command::new("./config")
			.current_dir(&openssl_path)
			.arg("shared")
			.status()?
			.success());
		// build
		assert!(Command::new("make")
			.current_dir(&openssl_path)
			.status()?
			.success());
	}

	// add OpenSSL libraries
	println!("cargo:rustc-link-search={}", openssl_path.display());
	println!("cargo:rustc-link-lib=static=crypto");
	println!("cargo:rustc-link-lib=static=ssl");

	// build balloon
	assert!(Command::new("scons")
		.current_dir("balloon")
		.args(&["VERBOSE=1", &format!("OPENSSL={}", openssl_path.display())])
		.status()?
		.success());

	// add Balloon library
	println!("cargo:rustc-link-search=balloon/build/libballoon");
	println!("cargo:rustc-link-lib=static=balloon");

	Ok(())
}
