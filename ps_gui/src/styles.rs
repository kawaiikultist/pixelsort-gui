use iced::{Background, Border, Color, Shadow, Theme, border::Radius, widget::container};

pub fn sort_control_pane(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();


    container::Style {
        text_color: Some(palette.background.base.text),
        background: Some(Background::Color(palette.background.base.color)),
        // border: palette.secondary.weak.color,
        border: Border { color: palette.background.strong.color, width: 2.0, ..Border::default() },
        shadow: Shadow::default(),
        snap: false,
    }
}


pub fn image_viewer_pane(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: None,
        background: Some(Background::Color(Color::BLACK)),
        border: Border { color: palette.background.strong.color, width: 2.0, ..Border::default() },
        shadow: Shadow::default(),
        snap: false
    }
}
