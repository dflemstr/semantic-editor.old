extern crate fern;
#[macro_use]
extern crate log;
extern crate rustbox;
extern crate time;

mod key;

use rustbox::{Color,Event,RustBox};

fn main() {
    setup_log();

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
