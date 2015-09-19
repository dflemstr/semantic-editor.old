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
    let dest_path = Path::new(&out_dir).join("version.rs");
    let mut f = try!(File::create(&dest_path));

    let git_version = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output().ok().map(|o| drop_last_byte(o.stdout));

    let git_commit_timestamp = Command::new("git")
        .args(&["show", "-s", "--format=%ct", "HEAD"])
        .output().ok().map(|o| drop_last_byte(o.stdout));

    try!(f.write_all(b"extern crate time;\n\npub fn version() -> &'static str {\n    "));

    match git_version {
        None => try!(f.write_all(b"env!(\"CARGO_PKG_VERSION\")")),
        Some(v) => {
            try!(f.write_all(b"concat!(env!(\"CARGO_PKG_VERSION\"), \"-"));
            try!(f.write_all(&v));
            try!(f.write_all(b"\")"));
        },
    };

    try!(f.write_all(b"\n}\n\npub fn committed_at() -> time::Timespec {\n    "));
    match git_commit_timestamp {
        None => try!(f.write_all(b"time::Timespec { sec: 0, nsec: 0 }")),
        Some(t) => {
            try!(f.write_all(b"time::Timespec { sec: "));
            try!(f.write_all(&t));
            try!(f.write_all(b", nsec: 0 }"));
        },
    };

    try!(f.write_all(b"\n}\n\npub fn target() -> &'static str {\n    \""));
    try!(f.write_all(try!(env::var("TARGET")).as_bytes()));
    try!(f.write_all(b"\"\n}\n"));

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
