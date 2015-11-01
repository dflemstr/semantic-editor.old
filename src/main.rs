#[macro_use]
extern crate clap;
extern crate fern;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate libc;
extern crate md5;
extern crate mio;
extern crate mioco;
extern crate nix;
extern crate rustbox;
extern crate time;

mod build;
mod key;
mod term;
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
    mioco::start(move |mioco| {
        let t = term::Term::new(mioco).unwrap();

        loop {
            let event = t.events_recv.read();
            info!("Event: {:?}", event);
        }
    });
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
        format: Box::new(|msg: &str, level: &log::LogLevel, location: &log::LogLocation| {
            format!("[{}] [{}] {}",
                    level, location.module_path(), msg)
        }),
        output: vec![fern::OutputConfig::file("se.log")],
        level: log::LogLevelFilter::Trace,
    };
    fern::init_global_logger(conf, log::LogLevelFilter::Trace).unwrap();
}
