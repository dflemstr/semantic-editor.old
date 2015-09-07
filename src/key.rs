use std::char;

use std::fmt;
use std::fmt::Write;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Press {
    pub ctrl: bool,
    pub alt: bool,
    pub symbol: Symbol,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Symbol {
    Char(char),
    Special(Special),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Special {
    Backspace,
    F(u32),
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
}

impl Press {
    pub fn from_raw(emod: u8, key: u16, character: u32) -> Option<Self> {
        let alt = emod == 0x01;

        if key == 0 {
            char::from_u32(character).map(|c| Press {
                ctrl: false,
                alt: alt,
                symbol: Symbol::Char(c),
            })
        } else if key < 0x20 {
            char::from_u32(key as u32 | 0b01100000).map(|c| Press {
                ctrl: true,
                alt: alt,
                symbol: Symbol::Char(c),
            })
        } else if key == 0x20 {
            Some(Press {
                ctrl: false,
                alt: alt,
                symbol: Symbol::Char(' '),
            })
        } else {
            let special = match key {
                0x7f => Some(Special::Backspace),
                0xffff => Some(Special::F(1)),
                0xfffe => Some(Special::F(2)),
                0xfffd => Some(Special::F(3)),
                0xfffc => Some(Special::F(4)),
                0xfffb => Some(Special::F(5)),
                0xfffa => Some(Special::F(6)),
                0xfff9 => Some(Special::F(7)),
                0xfff8 => Some(Special::F(8)),
                0xfff7 => Some(Special::F(9)),
                0xfff6 => Some(Special::F(10)),
                0xfff5 => Some(Special::F(11)),
                0xfff4 => Some(Special::F(12)),
                0xfff3 => Some(Special::Insert),
                0xfff2 => Some(Special::Delete),
                0xfff1 => Some(Special::Home),
                0xfff0 => Some(Special::End),
                0xffef => Some(Special::PageUp),
                0xffee => Some(Special::PageDown),
                0xffed => Some(Special::Up),
                0xffec => Some(Special::Down),
                0xffeb => Some(Special::Left),
                0xffea => Some(Special::Right),
                _ => None,
            };

            special.map(|s| Press {
                ctrl: false,
                alt: alt,
                symbol: Symbol::Special(s)
            })
        }
    }
}

impl fmt::Display for Press {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.ctrl {
            try!(f.write_str("C-"));
        }
        if self.alt {
            try!(f.write_str("M-"));
        }
        try!(self.symbol.fmt(f));

        Ok(())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Symbol::Char(c) => f.write_char(c),
            Symbol::Special(ref s) => fmt::Display::fmt(s, f),
        }
    }
}

impl fmt::Display for Special {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Special::Backspace => f.write_str("backspace"),
            Special::F(n) => write!(f, "f{}", n),
            Special::Insert => f.write_str("insert"),
            Special::Delete => f.write_str("delete"),
            Special::Home => f.write_str("home"),
            Special::End => f.write_str("end"),
            Special::PageUp => f.write_str("page-up"),
            Special::PageDown => f.write_str("page-down"),
            Special::Up => f.write_str("up"),
            Special::Down => f.write_str("down"),
            Special::Left => f.write_str("left"),
            Special::Right => f.write_str("right"),
        }
    }
}

#[test]
fn press_from_raw_char() {
    let result = Press::from_raw(0, 0, 'a' as u32);
    assert_eq!(result, Some(Press {
        ctrl: false,
        alt: false,
        symbol: Symbol::Char('a'),
    }));
}

#[test]
fn press_from_raw_alt_char() {
    let result = Press::from_raw(0x01, 0, 'a' as u32);
    assert_eq!(result, Some(Press {
        ctrl: false,
        alt: true,
        symbol: Symbol::Char('a'),
    }));
}

#[test]
fn press_from_raw_ctrl_char() {
    let result = Press::from_raw(0, '\x01' as u16, 0);
    assert_eq!(result, Some(Press {
        ctrl: true,
        alt: false,
        symbol: Symbol::Char('a'),
    }));
}

#[test]
fn press_from_raw_ctrl_alt_char() {
    let result = Press::from_raw(0x01, '\x01' as u16, 0);
    assert_eq!(result, Some(Press {
        ctrl: true,
        alt: true,
        symbol: Symbol::Char('a'),
    }));
}

#[test]
fn press_from_raw_special_f1() {
    let result = Press::from_raw(0, 0xffff, 0);
    assert_eq!(result, Some(Press {
        ctrl: false,
        alt: false,
        symbol: Symbol::Special(Special::F(1)),
    }));
}

#[test]
fn press_from_raw_special_right() {
    let result = Press::from_raw(0, 0xffff - 21, 0);
    assert_eq!(result, Some(Press {
        ctrl: false,
        alt: false,
        symbol: Symbol::Special(Special::Right),
    }));
}
