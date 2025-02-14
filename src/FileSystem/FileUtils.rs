use std::ffi::CString;
use std::os::raw::{c_char};
use crate::{FileInfo, FileList};

extern "C" {
  fn Copy(source: *const c_char, destination: *const c_char) -> bool;
  fn Cut(source: *const c_char, destination: *const c_char) -> bool;
  fn Rename(old_path: *const c_char, new_path: *const c_char) -> bool;
  fn MoveTo(source: *const c_char, destination: *const c_char) -> bool;
  fn Compress(source: *const c_char, destination: *const c_char) -> bool;
  fn Decompress(source: *const c_char, destination: *const c_char) -> bool;
  fn GetFiles(path: *const c_char) -> FileList;
  fn SearchFiles(path: *const c_char, query: *const c_char) -> FileList;
}

pub fn copy(source: &str, destination: &str) -> bool {
  let c_source = CString::new(source).expect("CString conversion failed");
  let c_destination = CString::new(destination).expect("CString conversion failed");
  unsafe { Copy(c_source.as_ptr(), c_destination.as_ptr()) }
}

pub fn cut(source: &str, destination: &str) -> bool {
  let c_source = CString::new(source).expect("CString conversion failed");
  let c_destination = CString::new(destination).expect("CString conversion failed");
  unsafe { Cut(c_source.as_ptr(), c_destination.as_ptr()) }
}

pub fn rename(old_path: &str, new_path: &str) -> bool {
  let c_old_path = CString::new(old_path).expect("CString conversion failed");
  let c_new_path = CString::new(new_path).expect("CString conversion failed");
  unsafe { Rename(c_old_path.as_ptr(), c_new_path.as_ptr()) }
}

pub fn move_to(source: &str, destination: &str) -> bool {
  let c_source = CString::new(source).expect("CString conversion failed");
  let c_destination = CString::new(destination).expect("CString conversion failed");
  unsafe { MoveTo(c_source.as_ptr(), c_destination.as_ptr()) }
}

pub fn compress(source: &str, destination: &str) -> bool {
  let c_source = CString::new(source).expect("CString conversion failed");
  let c_destination = CString::new(destination).expect("CString conversion failed");
  unsafe { Compress(c_source.as_ptr(), c_destination.as_ptr()) }
}

pub fn decompress(source: &str, destination: &str) -> bool {
  let c_source = CString::new(source).expect("CString conversion failed");
  let c_destination = CString::new(destination).expect("CString conversion failed");
  unsafe { Decompress(c_source.as_ptr(), c_destination.as_ptr()) }
}

pub fn get_files(path: &str) -> Vec<FileInfo> {
  let c_path = CString::new(path).expect("CString conversion failed");
  unsafe {
    let file_list = GetFiles(c_path.as_ptr());
    if file_list.files.is_null() || file_list.count == 0 {
      return vec![];
    }
    let files_slice = std::slice::from_raw_parts(file_list.files, file_list.count as usize);
    files_slice.to_vec()
  }
}

pub fn search_files(path: &str, query: &str) -> Vec<FileInfo> {
  let c_path = CString::new(path).expect("CString conversion failed");
  let c_query = CString::new(query).expect("CString conversion failed");
  unsafe {
    let file_list = SearchFiles(c_path.as_ptr(), c_query.as_ptr());
    if file_list.files.is_null() || file_list.count == 0 {
      return vec![];
    }
    let files_slice = std::slice::from_raw_parts(file_list.files, file_list.count as usize);
    files_slice.to_vec()
  }
}
