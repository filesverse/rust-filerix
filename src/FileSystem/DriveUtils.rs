use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_ulonglong};
use crate::utils::cstrUtils;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DriveUsage {
  pub used: c_ulonglong,
  pub total: c_ulonglong,
}

#[repr(C)]
#[derive(Clone)]
pub struct DriveInfo {
  pub device: String,
  pub status: String,
  pub mount_point: String,
  pub partition: String,
  pub fs_type: String,
  pub unmountable: bool,
}

impl From<&RawDriveInfo> for DriveInfo {
  fn from(raw: &RawDriveInfo) -> Self {
    Self {
      device: cstrUtils::to_string(&raw.device),
      status: cstrUtils::to_string(&raw.status),
      mount_point: cstrUtils::to_string(&raw.mount_point),
      partition: cstrUtils::to_string(&raw.partition),
      fs_type: cstrUtils::to_string(&raw.fs_type),
      unmountable: raw.unmountable,
    }
  }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawDriveInfo {
  pub device: [c_char; 256],
  pub status: [c_char; 256],
  pub mount_point: [c_char; 256],
  pub partition: [c_char; 256],
  pub fs_type: [c_char; 256],
  pub unmountable: bool,
}

#[repr(C)]
pub struct DriveList {
  items: *mut RawDriveInfo,
  count: usize,
}

extern "C" {
  fn GetDriveUsage(drive: *const c_char) -> DriveUsage;
  fn MountDrive(device: *const c_char) -> bool;
  fn UnmountDrive(device: *const c_char) -> bool;
  fn GetDeviceLabelOrUUID(device: *const c_char) -> *const c_char;
  fn GetDrives() -> DriveList;
}

pub fn get_drive_usage(drive: &str) -> Option<DriveUsage> {
  let c_drive = CString::new(drive).ok()?;
  Some(unsafe { GetDriveUsage(c_drive.as_ptr()) })
}

pub fn mount_drive(device: &str) -> bool {
  let c_device = CString::new(device).expect("CString conversion failed");
  unsafe { MountDrive(c_device.as_ptr()) }
}

pub fn unmount_drive(device: &str) -> bool {
  let c_device = CString::new(device).expect("CString conversion failed");
  unsafe { UnmountDrive(c_device.as_ptr()) }
}

pub fn get_device_label_or_uuid(device: &str) -> Option<String> {
  let c_device = CString::new(device).ok()?;
  unsafe {
    let ptr = GetDeviceLabelOrUUID(c_device.as_ptr());
    if ptr.is_null() {
      None
    } else {
      Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
    }
  }
}

pub fn get_drives() -> Vec<DriveInfo> {
  unsafe {
    let drive_list = GetDrives();
    if drive_list.items.is_null() || drive_list.count == 0 {
      return vec![];
    }

    let drives_slice = std::slice::from_raw_parts(drive_list.items, drive_list.count as usize);
    drives_slice.iter().map(DriveInfo::from).collect()
  }
}
