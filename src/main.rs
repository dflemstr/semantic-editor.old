extern crate rustbox;

use rustbox::{Color,Event,Key,RustBox};

fn main() {
    let rb = RustBox::init(Default::default()).unwrap();
    loop {
        match rb.poll_event(false).unwrap() {
            Event::KeyEvent(key) => {
                if key == Some(Key::Char('q')) {
                    break;
                }

                rb.clear();
                if let Some(k) = key {
                    let visual = fmt_emacs_key(k);
                    rb.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Black, &visual);
                }
            },
            _ => { }
        }
        rb.present();
    }
}

fn fmt_emacs_key(key: Key) -> String {
    match key {
        Key::Tab => String::from("tab"),
        Key::Enter => String::from("return"),
        Key::Esc => String::from("escape"),
        Key::Backspace => String::from("backspace"),
        Key::Right => String::from("right"),
        Key::Left => String::from("left"),
        Key::Up => String::from("up"),
        Key::Down => String::from("down"),
        Key::Delete => String::from("delete"),
        Key::Home => String::from("home"),
        Key::End => String::from("end"),
        Key::PageUp => String::from("prior"),
        Key::PageDown => String::from("next"),
        Key::Char(c) => c.to_string(),
        Key::Ctrl(c) => format!("C-{}", c),
        Key::F(n) => format!("f{}", n),
    }
}
