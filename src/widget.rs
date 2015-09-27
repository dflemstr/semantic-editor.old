use ui::UI;

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
    pub fn render(&self, ui: &UI) {
        match *self {
            Widget::Text { ref contents, .. } => ui.draw_text(contents),
            Widget::LinearLayout { .. } => panic!("Not implemented"),
        }
    }
}
