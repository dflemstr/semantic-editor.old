extern crate rustc_version;

use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
enum Error {
    Var(env::VarError),
    IO(io::Error),
}

fn main() {
    generate().unwrap();
}

fn generate() -> Result<(), Error> {
    let out_dir = try!(env::var("OUT_DIR"));
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    let mut f = try!(File::create(&dest_path));

    let git_version = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output().ok().map(|o| drop_last_byte(o.stdout));

    let git_commit_timestamp = Command::new("git")
        .args(&["show", "-s", "--format=%ct", "HEAD"])
        .output().ok().map(|o| drop_last_byte(o.stdout));

    let target = try!(env::var("TARGET"));
    let mut target_parts = target.split("-");
    let arch = target_parts.next().unwrap();
    let vendor = target_parts.next().unwrap();
    let system = target_parts.next().unwrap();
    let abi = target_parts.next().unwrap();

    let rust_version = format!("{}", rustc_version::version());

    try!(f.write_all(b"const COMMIT: Option<&'static str> = "));
    match git_version {
        None => try!(f.write_all(b"None")),
        Some(commit) => {
            try!(f.write_all(b"Some(\""));
            try!(f.write_all(&commit));
            try!(f.write_all(b"\")"));
        },
    };
    try!(f.write_all(b";\n\n"));

    try!(f.write_all(b"const COMMITTED_AT: Option<::time::Timespec> = "));
    match git_commit_timestamp {
        None => try!(f.write_all(b"None")),
        Some(t) => {
            try!(f.write_all(b"Some(::time::Timespec { sec: "));
            try!(f.write_all(&t));
            try!(f.write_all(b", nsec: 0 })"));
        },
    };
    try!(f.write_all(b";\n\n"));

    try!(f.write_all(b"const TARGET: &'static str = \""));
    try!(f.write_all(target.as_bytes()));
    try!(f.write_all(b"\";\n\n"));

    try!(f.write_all(b"const ARCH: &'static str = \""));
    try!(f.write_all(arch.as_bytes()));
    try!(f.write_all(b"\";\n\n"));

    try!(f.write_all(b"const VENDOR: &'static str = \""));
    try!(f.write_all(vendor.as_bytes()));
    try!(f.write_all(b"\";\n\n"));

    try!(f.write_all(b"const SYSTEM: &'static str = \""));
    try!(f.write_all(system.as_bytes()));
    try!(f.write_all(b"\";\n\n"));

    try!(f.write_all(b"const ABI: &'static str = \""));
    try!(f.write_all(abi.as_bytes()));
    try!(f.write_all(b"\";\n\n"));

    try!(f.write_all(b"const RUST_VERSION: &'static str = \""));
    try!(f.write_all(rust_version.as_bytes()));
    try!(f.write_all(b"\";\n"));

    Ok(())
}

fn drop_last_byte(mut bytes: Vec<u8>) -> Vec<u8> {
    bytes.pop();
    bytes
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<env::VarError> for Error {
    fn from(e: env::VarError) -> Error {
        Error::Var(e)
    }
}
