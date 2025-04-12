#[derive(Clone, Copy)]
pub enum BorderStyleType {
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    None,
    Hidden,
}

pub struct BorderStyle {
    top_style: Option<BorderStyleType>,
    bot_style: Option<BorderStyleType>,
    left_style: Option<BorderStyleType>,
    right_style: Option<BorderStyleType>,
}
impl BorderStyle {
    pub fn set_all(style: Option<BorderStyleType>) -> BorderStyle {
        BorderStyle {
            top_style: style,
            bot_style: style,
            left_style: style,
            right_style: style,
        }
    }
}

pub struct BorderRadius {
    top: Option<f32>,
    bot: Option<f32>,
    left: Option<f32>,
    right: Option<f32>,
}
impl BorderRadius {
    pub fn set_all(radius: f32) -> BorderRadius {
        BorderRadius {
            top: Some(radius),
            bot: Some(radius),
            left: Some(radius),
            right: Some(radius),
        }
    }
}

pub struct Border {
    style: BorderStyle,
    width: f32,
    rgb: (f32, f32, f32),
    radius: Option<BorderRadius>,
}
