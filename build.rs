use eyre::Result;

fn main() -> Result<()> {
	let bindings = bindgen::builder()
		.clang_args([
			"-Iinclude",
			"-fvisibility=default",
			"-D_HAVE_SQLITE_CONFIG_H"
		])
		.layout_tests(false)
		.header("vendor/sqlite3.h")
		.generate()?;
	bindings.write_to_file("src/sqlite.rs")?;

	cc::Build::new()
		.compiler("wasi-sdk-19.0/bin/clang")
		.archiver("wasi-sdk-19.0/bin/ar")
		.target("wasm32-wasi")
		.include("include")
		.define("_HAVE_SQLITE_CONFIG_H", "")
		.file("vendor/sqlite3.c")
		.compile("sqlite3");

	// Changed files:
	println!("cargo:rerun-if-changed=include/sqlite_cfg.h");
	println!("cargo:rerun-if-changed=vendor/sqlite3.c");
	println!("cargo:rerun-if-changed=vendor/sqlite3.h");

	// Libraries:
	println!("cargo:rustc-link-search=lib");
	println!("cargo:rustc-link-lib=clang_rt.builtins-wasm32");
	println!("cargo:rustc-link-lib=c");

	Ok(())
}
