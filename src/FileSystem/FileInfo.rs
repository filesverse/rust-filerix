use std::ffi::CStr;
use std::os::raw::{c_char, c_ulonglong};

#[repr(C)]
pub struct FileInfo {
  pub name: *mut c_char,
  pub file_type: *mut c_char,
  pub path: *mut c_char,
  pub size: c_ulonglong,
  pub is_directory: bool,
}

#[repr(C)]
pub struct FileList {
  pub files: *mut FileInfo,
  pub count: usize,
}

impl FileInfo {
  pub fn to_rust(&self) -> FileInfo {
    FileInfo {
      name: unsafe { CStr::from_ptr(self.name).to_string_lossy().into_owned() },
      file_type: unsafe { CStr::from_ptr(self.file_type).to_string_lossy().into_owned() },
      path: unsafe { CStr::from_ptr(self.path).to_string_lossy().into_owned() },
      size: self.size,
      is_directory: self.is_directory,
    }
  }
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub file_type: String,
    pub path: String,
    pub size: u64,
    pub is_directory: bool,
}
