use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use lazy_static::lazy_static;

extern "C" {
  fn StartFileMonitor(directory: *const c_char, callback: Option<unsafe extern "C" fn(*const c_char, *const c_char)>);
  fn StopFileMonitor();
}

lazy_static! {
  static ref CALLBACKS: Mutex<Option<Box<dyn Fn(String, String) + Send + 'static>>> = Mutex::new(None);
}

extern "C" fn file_event_callback(event_type: *const c_char, file_path: *const c_char) {
  let event_type_str = unsafe { CStr::from_ptr(event_type).to_string_lossy().into_owned() };
  let file_path_str = unsafe { CStr::from_ptr(file_path).to_string_lossy().into_owned() };

  if let Some(callback) = CALLBACKS.lock().unwrap().as_ref() {
    callback(event_type_str, file_path_str);
  }
}

pub fn start_file_monitor<F>(directory: &str, callback: F)
where
  F: Fn(String, String) + Send + 'static,
{
  let c_directory = CString::new(directory).expect("CString conversion failed");

  let mut cb = CALLBACKS.lock().unwrap();
  *cb = Some(Box::new(callback));

  unsafe {
    StartFileMonitor(c_directory.as_ptr(), Some(file_event_callback));
  }
}

pub fn stop_file_monitor() {
  unsafe {
    StopFileMonitor();
  }

  let mut cb = CALLBACKS.lock().unwrap();
  *cb = None;
}
