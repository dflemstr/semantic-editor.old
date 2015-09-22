#[macro_use]
extern crate clap;
extern crate fern;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate md5;
extern crate rustbox;
extern crate time;

mod key;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
}

mod os {
    #[cfg(target_os="linux")]
    const NAME: &'static str = "linux";

    #[cfg(target_os="macos")]
    const NAME: &'static str = "osx";

    // Other OS'es not supported right now

    pub fn name() -> &'static str {
        NAME
    }
}

fn main() {
    setup_log();

    let matches = parse_cli_args();

    if let Some(u) = matches.subcommand_matches("update") {
        update(&env::current_exe().unwrap().as_path())
    } else {
        edit()
    }
}

fn update(executable_path: &Path) {
    use hyper::Client;
    use hyper::header;
    use hyper::status;

    use std::os::unix::fs::PermissionsExt;

    let hash = md5sum(&mut fs::File::open(executable_path).unwrap());
    let etag = header::EntityTag::strong(hash);

    let client = Client::new();

    let mut res = client.get("http://dflemstr.name/se")
        .header(header::IfNoneMatch::Items(vec![etag]))
        .header(header::UserAgent(os::name().to_owned()))
        .send().unwrap();

    if res.status.is_success() {
        let old_path = executable_path.with_file_name(&format!("se-{}", version::version()));
        fs::rename(executable_path, &old_path).unwrap();
        println!("Note: old version saved as {}", old_path.to_str().unwrap());

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

fn edit() {
    use rustbox::{Color,Event,RustBox};

    let rb = RustBox::init(rustbox::InitOptions {
        input_mode: rustbox::InputMode::AltMouse,
        buffer_stderr: true,
    }).unwrap();

    loop {
        match rb.poll_event(true).unwrap() {
            Event::KeyEventRaw(emod, key, character) => {
                if let Some(press) = key::Press::from_raw(emod, key, character) {
                    debug!("Key event: {}", press);

                    if press.symbol == key::Symbol::Char('q') {
                        return;
                    }

                    rb.clear();
                    let visual = format!("{}", press);
                    rb.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, &visual);
                } else {
                    warn!("Unhandled raw key event {} {} {}", emod, key, character);
                }
            },
            Event::KeyEvent(_) => unreachable!("got parsed key event in raw mode"),
            Event::ResizeEvent(w, h) => debug!("Resize event: {}×{}", w, h),
            Event::MouseEvent(m, x, y) => debug!("Mouse event: {:?} at {},{}", m, x, y),
            Event::NoEvent => debug!("No event"),
        }
        rb.present();
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

fn parse_cli_args<'n, 'a>() -> clap::ArgMatches<'n, 'a> {
    let about =
        &format!("The Semantic Editor — Next generation editing\
                \n\
                \nBUILD DETAILS:\
                \n    Target: {}\
                \n    Committed: {}",
                 version::target(),
                 time::at_utc(version::committed_at()).rfc822());
    clap_app!(SemanticEditor =>
        (@setting GlobalVersion)
        (version: version::version())
        (about: about)
        (@arg files: ... "File(s) to edit")
        (@subcommand update =>
            (about: "checks and downloads updates to se")
        )
    ).get_matches()
}

fn setup_log() {
    let conf = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{}][{}] {}",
                    time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(), level, msg)
        }),
        output: vec![fern::OutputConfig::file("se.log")],
        level: log::LogLevelFilter::Trace,
    };
    fern::init_global_logger(conf, log::LogLevelFilter::Trace).unwrap();
}
