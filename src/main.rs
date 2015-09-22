extern crate fern;
#[macro_use]
extern crate log;
extern crate rustbox;
extern crate time;

mod key;

use rustbox::{Color,Event,Key,RustBox};

fn main() {
    setup_log();

    let rb = RustBox::init(rustbox::InitOptions {
        input_mode: rustbox::InputMode::AltMouse,
        buffer_stderr: true,
    }).unwrap();

    loop {
        let intent;
        match rb.peek_event(time::Duration::milliseconds(1), true).unwrap() {
            Event::KeyEventRaw(emod, key, character) => {
                if let Some(press) = key::Press::from_raw(emod, key, character) {
                    debug!("Key event: {}", press);
                    intent = intent_from_press(press);
                } else {
                    warn!("Unhandled raw key event {} {} {}", emod, key, character);
                    intent = None;
                }
            },
            Event::KeyEvent(_) => panic!("Got parsed key event in raw mode"),
            Event::ResizeEvent(w, h) => debug!("Resize event: {}×{}", w, h),
            Event::MouseEvent(m, x, y) => debug!("Mouse event: {:?} at {},{}", m, x, y),
            Event::NoEvent => trace!("No event"),
        }

        rb.clear();

        for y in 0..rb.height() {
            for x in 0..rb.width() {
                rb.print_char(x, y, rustbox::RB_NORMAL, Color::White, Color::Black, ' ');
            }
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
        level: log::LogLevelFilter::Debug,
    };
    fern::init_global_logger(conf, log::LogLevelFilter::Trace).unwrap();
}
