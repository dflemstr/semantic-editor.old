use build;

use hyper::Client;
use hyper::header;
use hyper::status;

use md5;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

use std::os::unix::fs::PermissionsExt;

pub fn download() {
    download_to(&env::current_exe().unwrap().as_path())
}

fn download_to(executable_path: &Path) {
    let hash = md5sum(&mut fs::File::open(executable_path).unwrap());
    let etag = header::EntityTag::strong(hash);

    let client = Client::new();

    let mut res = client.get("http://dflemstr.name/se")
        .header(header::IfNoneMatch::Items(vec![etag]))
        .header(header::UserAgent(build::user_agent()))
        .send().unwrap();

    if res.status.is_success() {
        if fs::metadata(executable_path).is_ok() {
            let old_path = executable_path.with_file_name(&format!("se-{}", build::version()));
            fs::rename(executable_path, &old_path).unwrap();
            println!("Note: old version saved as {}", old_path.to_str().unwrap());
        }

        println!("Downloading new version...");
        io::copy(&mut res, &mut fs::File::create(executable_path).unwrap()).unwrap();

        let mut permissions = fs::metadata(executable_path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(executable_path, permissions).unwrap();

        println!("Update successful");
    } else if res.status == status::StatusCode::NotModified {
        println!("No update available");
    } else {
        println!("Got status {}", res.status);
    }
}

fn md5sum<R>(input: &mut R) -> String where R: io::Read {
    let mut context = md5::Context::new();
    io::copy(input, &mut context).unwrap();

    // TODO: this is horribly inefficient probably...
    context.compute().iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}
