use ui::{Color,UI};

pub enum Widget {
    Text {
        contents: String,
    },
    LinearLayout {
        orientation: Orientation,
        contents: Vec<(u32, Widget)>
    },
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Widget {
    pub fn render(&self, ui: &mut UI) {
        ui.style().background_color = Color::Blue;
        ui.style().foreground_color = Color::White;
        ui.style().bold = true;
        match *self {
            Widget::Text { ref contents, .. } => ui.draw_text(contents),
            Widget::LinearLayout { .. } => panic!("Not implemented"),
        }
    }
}
