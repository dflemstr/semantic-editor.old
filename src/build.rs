// Contains:
// const VERSION: &'static str = "1.2.3-abcdefg";
// const COMMITTED_AT: ::time::Timespec = ...;
// const TARGET: &'static str = "x86_64-unknown-linux-gnu";
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

#[cfg(target_os="linux")]
const OS_NAME: &'static str = "linux";

#[cfg(target_os="macos")]
const OS_NAME: &'static str = "osx";

pub fn app_name() -> &'static str {
    "semantic-editor"
}

pub fn os_name() -> &'static str {
    OS_NAME
}

pub fn user_agent() -> String {
    format!("{}/{} {}", app_name(), version(), os_name())
}

pub fn version() -> &'static str {
    VERSION
}

pub fn committed_at() -> ::time::Timespec {
    COMMITTED_AT
}

pub fn target() -> &'static str {
    TARGET
}
