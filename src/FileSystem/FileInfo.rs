use std::os::raw::{c_char, c_ulonglong};

#[repr(C)]
pub struct FileInfoRaw {
  pub name: *mut c_char,
  pub file_type: *mut c_char,
  pub path: *mut c_char,
  pub size: c_ulonglong,
  pub is_directory: bool,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
  pub name: String,
  pub file_type: String,
  pub path: String,
  pub size: u64,
  pub is_directory: bool,
}

#[repr(C)]
pub struct FileList {
  pub files: *mut FileInfoRaw,
  pub count: usize,
}