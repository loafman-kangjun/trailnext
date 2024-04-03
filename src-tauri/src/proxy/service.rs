use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn install() {
    let label: ServiceLabel = "com.example.my-service".parse().unwrap();
}