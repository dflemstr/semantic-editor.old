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
        .output().ok()
        .map(|o| String::from_utf8(drop_last_byte(o.stdout)).unwrap());

    let git_commit_timestamp = Command::new("git")
        .args(&["show", "-s", "--format=%ct", "HEAD"])
        .output().ok()
        .map(|o| String::from_utf8(drop_last_byte(o.stdout)).unwrap());

    let target = try!(env::var("TARGET"));
    let mut target_parts = target.split("-");
    let arch = target_parts.next().unwrap();
    let vendor = target_parts.next().unwrap();
    let system = target_parts.next().unwrap();
    let abi = target_parts.next();

    let rust_version = format!("{}", rustc_version::version());

    try!(write!(f, "const COMMIT: Option<&'static str> = "));
    match git_version {
        None => try!(write!(f, "None")),
        Some(commit) => try!(write!(f, "Some(\"{}\")", commit)),
    };
    try!(write!(f, ";\n\n"));

    try!(write!(f, "const COMMITTED_AT: Option<::time::Timespec> = "));
    match git_commit_timestamp {
        None => try!(write!(f, "None")),
        Some(t) => try!(write!(f, "Some(::time::Timespec {{ sec: {}, nsec: 0 }})", t)),
    };
    try!(write!(f, ";\n\n"));

    try!(write!(f, "const TARGET: &'static str = \"{}\";\n\n", target));
    try!(write!(f, "const ARCH: &'static str = \"{}\";\n\n", arch));
    try!(write!(f, "const VENDOR: &'static str = \"{}\";\n\n", vendor));
    try!(write!(f, "const SYSTEM: &'static str = \"{}\";\n\n", system));

    try!(write!(f, "const ABI: Option<&'static str> = "));
    match abi {
        Some(a) => try!(write!(f, "Some(\"{}\")", a)),
        None => try!(write!(f, "None")),
    }
    try!(write!(f, ";\n\n"));

    try!(write!(f, "const RUST_VERSION: &'static str = \"{}\";\n", rust_version));

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
