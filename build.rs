use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
  let lib = pkg_config::probe_library("filerix")
    .expect("Failed to find libfilerix using pkg-config");

  for path in &lib.link_paths {
    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path.display());
  }

  println!("cargo:rustc-link-lib=dylib=filerix");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  let include_path = String::from_utf8(
    Command::new("pkg-config")
      .args(&["--cflags-only-I", "filerix"])
      .output()
      .expect("Failed to run pkg-config")
      .stdout,
  )
  .expect("Invalid UTF-8 output from pkg-config")
  .trim()
  .replace("-I", "");

  let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .clang_arg(format!("-I{}", include_path))
    .generate()
    .expect("Failed to generate bindings");

  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
