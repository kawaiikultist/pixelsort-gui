use crate::{app::App, msgs::*};
use iced::{
    Element, Length, Pixels, alignment::{Horizontal, Vertical}, widget::{column, container, grid, pick_list, row, rule, slider, text}
};
use pixelsort::prelude::{SortKey, SortPath};

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


pub fn sort_key_pick_list(app: &App) -> Element<'_, Message> {
    let choices = SortKey::get_list();
    column![
        row![
            text("Key").align_x(Horizontal::Left),
            container(pick_list(choices, Some(app.get_sort_key()), |v| Message::Param(ParamMessage::SortKeyChanged(v)))).align_x(Horizontal::Right).width(Length::Fill)
        ].width(Length::Fill),
        rule::horizontal(RULE_THICKNESS_H)
    ].spacing(SPACING_V).into()
}


pub fn sort_path(app: &App) -> Element<'_, Message> {
    let choices = SortPath::get_list();

    let path_conf: Element<'_, Message>  = {
        let app_sort_path = app.get_sort_path();
        match app_sort_path {
            SortPath::Linear => column![].height(0).into(),
            SortPath::Radial { x_offset: _, y_offset: _ } => radial_path_config(app),
            SortPath::Blocks { x_size: _, y_size: _ } => blocks_path_config(app),
        }
    };

    column![
        row![
            text("Path").align_x(Horizontal::Left),
            container(pick_list(choices, Some(app.get_sort_path()), |v| Message::Param(ParamMessage::SortPathChanged(v)))).align_x(Horizontal::Right).width(Length::Fill),
        ],
        path_conf,
        rule::horizontal(RULE_THICKNESS_H),
    ].spacing(SPACING_V).into()
}


fn radial_path_config(app: &App) -> Element<'_, Message> {
    let offset = app.get_sort_path().get_radial_offset().expect("Could not get radial offset!");
    let image_size = app.get_image_size();
    let half_x = image_size[0] as i32 / 2;
    let half_y = image_size[1] as i32 / 2;

    // TODO: Maybe switch out sliders for iced_aw::number_input.
    column![
        row![
            text("X Offset").align_x(Horizontal::Left),
            slider(-half_x..=half_x, offset[0], move |v| Message::Param(ParamMessage::SortPathChanged(SortPath::Radial { x_offset: v, y_offset: offset[1] }))),
            text(offset[0]).align_x(Horizontal::Right),
        ].spacing(SPACING_H).align_y(Vertical::Center),

        row![
            text("Y Offset").align_x(Horizontal::Left),
            slider(-half_y..=half_y, offset[1], move |v| Message::Param(ParamMessage::SortPathChanged(SortPath::Radial { x_offset: offset[0], y_offset: v }))),
            text(offset[1]).align_x(Horizontal::Right),
        ].spacing(SPACING_H).align_y(Vertical::Center),
    ].spacing(SPACING_V).into()
}


fn blocks_path_config(app: &App) -> Element<'_, Message> {
    let size = app.get_sort_path().get_block_size().expect("Could not get block size!");
    let image_size = app.get_image_size();

    // TODO: Maybe switch out sliders for iced_aw::number_input.
    column![
        row![
            text("Block W").align_x(Horizontal::Left).width(Length::Fixed(60.0)),
            slider(1..=image_size[0], size[0], move |v| Message::Param(ParamMessage::SortPathChanged(SortPath::Blocks { x_size: v, y_size: size[1] }))),
            text(size[0]).align_x(Horizontal::Right),
        ].spacing(SPACING_H).align_y(Vertical::Center),

        row![
            text("Block H").align_x(Horizontal::Left).width(Length::Fixed(60.0)),
            slider(1..=image_size[1], size[1], move |v| Message::Param(ParamMessage::SortPathChanged(SortPath::Blocks { x_size: size[0], y_size: v }))),
            text(size[1]).align_x(Horizontal::Right),
        ].spacing(SPACING_H).align_y(Vertical::Center),
    ].into()
}
