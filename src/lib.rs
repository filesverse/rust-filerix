#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[link(name = "filerix", kind = "dylib")]
extern "C" {}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod FileSystem;
pub mod Listeners;
pub mod utils;