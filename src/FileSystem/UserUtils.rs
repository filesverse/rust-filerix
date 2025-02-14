use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

extern "C" {
  fn GetUserName() -> *const c_char;
  fn ChangePermissions(path: *const c_char, mode: c_int) -> bool;
}


pub fn get_user_name() -> Option<String> {
  unsafe {
    let ptr = GetUserName();
    if ptr.is_null() {
      return None;
    }
    Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
  }
}

pub fn change_permissions(path: &str, mode: i32) -> bool {
  let c_path = CString::new(path).expect("CString conversion failed");
  unsafe { ChangePermissions(c_path.as_ptr(), mode) }
}

