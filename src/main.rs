extern crate fern;
#[macro_use]
extern crate log;
extern crate rustbox;
extern crate time;

mod key;

use rustbox::{Color,Event,Key,RustBox};

fn main() {
    setup_log();

    let rb = RustBox::init(Default::default()).unwrap();
    loop {
        match rb.poll_event(true).unwrap() {
            Event::KeyEventRaw(emod, key, character) => {
                if let Some(press) = key::Press::from_raw(emod, key, character) {

                    if press.symbol == key::Symbol::Char('q') {
                        return;
                    }

                    rb.clear();
                    let visual = format!("{}", press);
                    rb.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, &visual);
                } else {
                    warn!("Unhandled key event {} {} {}", emod, key, character);
                }
            },
            _ => warn!("Unhandled event"),
        }
        rb.present();
    }
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
