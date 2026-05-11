use crate::msgs::*;
use iced::{
    Element, alignment::{Horizontal, Vertical}, Pixels,
    widget::{row, column, text, rule, slider}
};

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                               Param Controls                                │
// └─────────────────────────────────────────────────────────────────────────────┘
const SPACING_H: Pixels = Pixels(10.0);
const SPACING_V: Pixels = Pixels(20.0);
const RULE_THICKNESS_H: Pixels = Pixels(2.0);


pub fn param_slider(label: &str, min: f32, max: f32, value: f32, on_change: impl Fn(f32) -> Message + 'static) -> Element<'_, Message> {
    column![
        row![
            text(label).align_x(Horizontal::Left),
            slider(min..=max, value, on_change),
            text(value).align_x(Horizontal::Right)
        ].spacing(SPACING_H).align_y(Vertical::Center),
        rule::horizontal(RULE_THICKNESS_H)
    ].spacing(SPACING_V).into()
}



