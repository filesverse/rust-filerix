use std::ffi::{CStr, c_char};

pub fn to_string(cstr: &[c_char; 256]) -> String {
  unsafe {
    CStr::from_ptr(cstr.as_ptr())
      .to_string_lossy()
      .into_owned()
  }
}

pub fn to_string_from_ptr(cstr: *mut c_char) -> String {
  if cstr.is_null() {
    return String::new();
  }

  unsafe {
    CStr::from_ptr(cstr)
      .to_string_lossy()
      .into_owned()
  }
}
