use std::path::PathBuf;

use iced::{Task, application};
use rfd::FileDialog;
use crate::app::App;


mod app;

fn main() -> iced::Result {
 
    let mut file_path = PathBuf::new();
    // HACK: Uses a loop to ensure the app will have a valid file.
    // Should probably provide some form of feedback to user if this fails.
    loop {
        if let Some(path) = FileDialog::new().add_filter("Image", &["png"]).pick_file() {
            file_path = path;
            break;
        }
    }


    // application(move || (App { file_path: file_path.clone(), ..Default::default() }, Task::none()), App::update, App::view)
    application(move || (App::new(file_path.clone()), Task::none()), App::update, App::view)
        .title("Pixelsorter")
        .run()

}
