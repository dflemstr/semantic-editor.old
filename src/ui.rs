use rustbox;

pub struct UI<'a> {
    target: &'a rustbox::RustBox,
    pub clip: Rect,
    pub style: Style,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Style {
    pub bold: bool,
    pub underline: bool,
    pub reverse: bool,
    pub foreground_color: Color,
    pub background_color: Color,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rect {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl<'a> UI<'a> {
    pub fn new(rustbox: &'a rustbox::RustBox) -> Self {
        UI {
            target: rustbox,
            clip: Rect {
                left: 0,
                top: 0,
                right: rustbox.width(),
                bottom: rustbox.height(),
            },
            style: Style {
                bold: false,
                underline: false,
                reverse: false,
                foreground_color: Color::Default,
                background_color: Color::Default,
            },
        }
    }

    pub fn fill(&self) {
        for y in self.clip.top..self.clip.bottom {
            for x in self.clip.left..self.clip.right {
                self.plot(x, y);
            }
        }
    }

    pub fn draw_text(&self, contents: &str) {
        self.fill();

        let mut col = 0;
        let mut row = 0;

        for c in contents.chars() {
            // TODO: handle newlines, rtl, wrap, etc
            self.set(self.clip.left + col, self.clip.top + row, c);
            col += 1;
        }
    }

    fn set(&self, x: usize, y: usize, c: char) {
        let rb_style = self.style.to_rb_style();
        let rb_fg_color = self.style.to_rb_fg_color();
        let rb_bg_color = self.style.to_rb_bg_color();
        self.target.print_char(x, y, rb_style, rb_fg_color, rb_bg_color, c);
    }

    fn plot(&self, x: usize, y: usize) {
        self.set(x, y, ' ');
    }
}

impl Style {
    fn to_rb_fg_color(&self) -> rustbox::Color {
        self.foreground_color.to_rb_color()
    }

    fn to_rb_bg_color(&self) -> rustbox::Color {
        self.background_color.to_rb_color()
    }

    fn to_rb_style(&self) -> rustbox::Style {
        let mut result = rustbox::RB_NORMAL;

        if self.bold {
            result = result | rustbox::RB_BOLD;
        }

        if self.underline {
            result = result | rustbox::RB_UNDERLINE;
        }

        if self.reverse {
            result = result | rustbox::RB_REVERSE;
        }

        result
    }
}

impl Color {
    fn to_rb_color(&self) -> rustbox::Color {
        match *self {
            Color::Default => rustbox::Color::Default,
            Color::Black   => rustbox::Color::Black,
            Color::Red     => rustbox::Color::Red,
            Color::Green   => rustbox::Color::Green,
            Color::Yellow  => rustbox::Color::Yellow,
            Color::Blue    => rustbox::Color::Blue,
            Color::Magenta => rustbox::Color::Magenta,
            Color::Cyan    => rustbox::Color::Cyan,
            Color::White   => rustbox::Color::White,
        }
    }
}
