#[macro_use]
extern crate clap;
extern crate fern;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate md5;
extern crate mio;
extern crate mioco;
extern crate rustbox;
extern crate time;

mod build;
mod key;
mod ui;
mod update;
mod widget;

fn main() {
    setup_log();

    let matches = parse_cli_args();

    if let Some(_) = matches.subcommand_matches("update") {
        update::download()
    } else {
        edit()
    }
}

fn edit() {
    use std::io::{Read, Write};
    mioco::start(move |mioco| {
        let mut stdin = mioco.wrap(mio::fd::stdin());
        let stdout = mioco.wrap(mio::fd::stdout());

        let mut buf = [0u8; 1024 * 16];

        loop {
            let size = try!(stdin.read(&mut buf));
            if size == 0 {
                return Ok(()); // eof
            }
            try!(stdin.write_all(&mut buf[0..size]));
        }
    });
    println!("Exiting!");
}

fn parse_cli_args<'n, 'a>() -> clap::ArgMatches<'n, 'a> {
    let build_info = build::info().into_iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n    ");
    let about =
        &format!("The Semantic Editor — Next generation editing\
                \n\
                \nBUILD INFORMATION:\n    {}",
                 build_info);
    clap_app!(SemanticEditor =>
        (@setting GlobalVersion)
        (version: &build::version())
        (about: about)
        (@arg files: ... "File(s) to edit")
        (@subcommand update =>
            (about: "Checks for and downloads updates to this program")
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
