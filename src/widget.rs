use ui::{Color,Rect,UI};

pub enum Widget {
    Text {
        contents: String,
    },
    LinearLayout {
        orientation: Orientation,
        children: Vec<(usize, Widget)>
    },
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Widget {
    pub fn render(&self, ui: &mut UI) {
        match *self {
            Widget::Text { ref contents, .. } => {
                ui.style().background_color = Color::Blue;
                ui.style().foreground_color = Color::White;
                ui.style().bold = true;
                ui.draw_text(contents);
            },
            Widget::LinearLayout { ref children, ref orientation, .. } => {
                let total_clip = ui.clip().clone();
                let total_weight = children.iter()
                    .map(|c| c.0)
                    .fold(0, |x, y| x + y);
                let width = (total_clip.right - total_clip.left) as f64;
                let height = (total_clip.bottom - total_clip.top) as f64;

                let mut taken_weight = 0;
                for &(weight, ref child) in children {
                    let lower_ratio = taken_weight as f64 / total_weight as f64;
                    let upper_ratio = (taken_weight + weight) as f64 / total_weight as f64;

                    let sub_clip = match *orientation {
                        Orientation::Horizontal => Rect {
                            top: total_clip.top,
                            bottom: total_clip.bottom,
                            left: (lower_ratio * width) as usize,
                            right: (upper_ratio * width) as usize,
                        },
                        Orientation::Vertical => Rect {
                            top: (lower_ratio * height) as usize,
                            bottom: (upper_ratio * height) as usize,
                            left: total_clip.left,
                            right: total_clip.right,
                        },
                    };

                    let mut sub_ui = ui.push();
                    *sub_ui.clip() = sub_clip.clone();

                    child.render(&mut sub_ui);
                    sub_ui.draw_text(&format!("{:?}", sub_clip));

                    taken_weight += weight;
                }
            },
        }
    }
}
