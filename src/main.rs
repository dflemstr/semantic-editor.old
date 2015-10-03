#[macro_use]
extern crate clap;
extern crate fern;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate md5;
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
    use rustbox::{Event,RustBox};
    use ui::UI;
    use widget::{Orientation,Widget};

    let rb = RustBox::init(rustbox::InitOptions {
        input_mode: rustbox::InputMode::AltMouse,
        buffer_stderr: true,
    }).unwrap();

    let mut widget = Widget::Text {
        contents: "do something".to_owned(),
    };

    loop {
        match rb.poll_event(true).unwrap() {
            Event::KeyEventRaw(emod, key, character) => {
                if let Some(press) = key::Press::from_raw(emod, key, character) {
                    debug!("Key event: {}", press);

                    if press.symbol == key::Symbol::Char('q') {
                        return;
                    } else {
                        widget = Widget::Text {
                            contents: format!("key press: {}", press),
                        };
                    }
                } else {
                    warn!("Unhandled raw key event {} {} {}", emod, key, character);
                }
            },
            Event::KeyEvent(_) => unreachable!("got parsed key event in raw mode"),
            Event::ResizeEvent(w, h) => {
                debug!("Resize event: {}×{}", w, h);
                widget = Widget::Text {
                    contents: format!("resize: {}×{}", w, h),
                };
            },
            Event::MouseEvent(m, x, y) => {
                debug!("Mouse event: {:?} at {},{}", m, x, y);
                widget = Widget::Text {
                    contents: format!("mouse event: {:?} at {},{}", m, x, y),
                };
            },
            Event::NoEvent => trace!("No event"),
        };

        rb.clear();
        widget.render(&UI::new(&rb));
        rb.present();
    }
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
