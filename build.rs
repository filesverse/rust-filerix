fn main() {
  let lib = pkg_config::probe_library("filerix")
    .expect("Failed to find libfilerix using pkg-config");

  for path in lib.link_paths {
    println!("cargo:rustc-link-search=native={}", path.display());

    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path.display());
  }

  println!("cargo:rustc-link-lib=filerix");
}
