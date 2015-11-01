// Contains: COMMIT, COMMITTED_AT, TARGET, ARCH, VENDOR, SYSTEM, ABI,
// RUST_VERSION
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

#[cfg(target_os="linux")]
const OS_NAME: &'static str = "linux";

#[cfg(target_os="macos")]
const OS_NAME: &'static str = "osx";

pub fn app_name() -> &'static str {
    "semantic-editor"
}

pub fn info() -> Vec<(String, String)> {
    let mut result = Vec::new();

    result.push((app_name().to_owned(), version()));
    result.push(("rust".to_owned(), RUST_VERSION.to_owned()));
    if let Some(commit) = COMMIT {
        result.push(("git-commit".to_owned(), commit.to_owned()));
    }
    result.push(("os".to_owned(), OS_NAME.to_owned()));
    result.push(("arch".to_owned(), ARCH.to_owned()));
    result.push(("vendor".to_owned(), VENDOR.to_owned()));

    if let Some(abi) = ABI {
        result.push(("abi".to_owned(), abi.to_owned()));
    }

    result
}

pub fn user_agent() -> String {
    info().into_iter()
        .map(|(k, v)| format!("{}/{}", k, v))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn version() -> String {
    let cargo_version = env!("CARGO_PKG_VERSION");
    match COMMIT {
        Some(commit) => format!("{}-{}", cargo_version, commit),
        None => cargo_version.to_owned(),
    }
}
