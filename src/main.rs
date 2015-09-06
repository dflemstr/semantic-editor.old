extern crate rustty;

use rustty::{Event,Terminal};

fn main() {
    let mut term = Terminal::new().unwrap();
    loop {
        let evt = term.get_event(-1).unwrap();
        if let Some(Event::Key(ch)) = evt {
            match ch {
                'q' => break,
                _ => (),
            }
        }
        term.swap_buffers().unwrap();
    }
}
