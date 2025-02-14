use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use lazy_static::lazy_static;

extern "C" {
    fn StartDriveMonitor(callback: extern "C" fn(*const c_char, *const c_char));
    fn StopDriveMonitor();
}

lazy_static! {
    static ref CALLBACKS: Mutex<Option<Box<dyn Fn(String, String) + Send + 'static>>> = Mutex::new(None);
}

extern "C" fn drive_event_callback(action: *const c_char, device: *const c_char) {
    let action_str = unsafe { CStr::from_ptr(action).to_string_lossy().into_owned() };
    let device_str = unsafe { CStr::from_ptr(device).to_string_lossy().into_owned() };

    if let Some(callback) = CALLBACKS.lock().unwrap().as_ref() {
        callback(action_str, device_str);
    }
}

pub fn start_drive_monitor<F>(callback: F)
where
    F: Fn(String, String) + Send + 'static,
{
    let mut cb = CALLBACKS.lock().unwrap();
    *cb = Some(Box::new(callback));
    
    unsafe {
        StartDriveMonitor(drive_event_callback);
    }
}

pub fn stop_drive_monitor() {
    unsafe {
        StopDriveMonitor();
    }

    let mut cb = CALLBACKS.lock().unwrap();
    *cb = None;
}
