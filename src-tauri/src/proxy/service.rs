use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn install() {
    let label: ServiceLabel = "top.loafman.trail.clash".parse().unwrap();

    let manager = <dyn ServiceManager>::native().expect("Failed to detect management platform");

    manager
        .install(ServiceInstallCtx {
            label: label.clone(),
            program: PathBuf::from("D:/TrailNext/src-tauri/src/proxy/core/clash.exe"),
            args: vec![OsString::from("--some-arg")],
            contents: None, // Optional String for system-specific service content.
            username: None, // Optional String for alternative user to run service.
            working_directory: None, // Optional String for the working directory for the service process.
            environment: None, // Optional list of environment variables to supply the service process.
        })
        .expect("Failed to install");

    manager
        .start(ServiceStartCtx {
            label: label.clone(),
        })
        .expect("Failed to start");
}
