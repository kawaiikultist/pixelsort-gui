use iced::{Task, application};
use rfd::FileDialog;
use crate::app::App;

mod app;
mod msgs;
mod widgets;
mod styles;
mod utils;

fn main() -> iced::Result {
    let input_image = {
        if let Some(path) = FileDialog::new().add_filter("Image", &["png"]).pick_file()
            && let Ok(image) = pixelsort::open(path) {
                image
        } else {
            pixelsort::utils::get_test_image(512, 512)
        }
    };


    application(move || (App::new(input_image.clone()), Task::none()), App::update, App::view)
        .title("KolorSort")
        .theme(App::THEME)
        .run()
}
