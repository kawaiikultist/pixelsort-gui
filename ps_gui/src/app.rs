use std::{path::PathBuf};

use iced::{
    Application, Element, widget::{Image, button, column, combo_box, image, image::Handle, pick_list, row, text}
};
use pixelsort::{RgbImage, prelude::*};
use rfd::FileDialog;


#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum PathSelection {
    #[default]
    Linear,
    Radial,
    Blocks,
}

impl PathSelection {
    fn get_sort_path(&self) -> SortPath {
        match self {
            Self::Linear => { SortPath::Linear },
            Self::Radial => { SortPath::Radial { x_offset: 0, y_offset: 0 }},
            Self::Blocks => {SortPath::Blocks { x_size: 64, y_size: 64 }},
        }
    }
}


impl std::fmt::Display for PathSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Linear => "Linear",
            Self::Radial => "Radial",
            Self::Blocks => "Blocks",
        })
    }
}


#[derive(Clone, Debug)]
pub enum Message {
    SortPathSelected(PathSelection),
    RunSorting,
    SaveFile,
}


#[derive(Default)]
pub struct App {
    pub file_path: PathBuf,
    
    sort_path: Option<PathSelection>,

    output: Option<RgbImage>
}


impl App {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path, ..Default::default()}
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SortPathSelected(ps) => { self.sort_path = Some(ps); },
            Message::RunSorting => {
                let image = pixelsort::open(self.file_path.as_path())
                    .expect("Could not open image!").to_rgb8();

                let path = {
                    if let Some(selection) = self.sort_path {
                        selection.get_sort_path()
                    } else {
                        SortPath::Linear
                    }
                };

                self.output = Some(Pixelsorter::new(image)
                    .set_path(path)
                    .set_key(SortKey::Luma)
                    .run());
            },
            Message::SaveFile => {
                let desired_save_path = FileDialog::new().add_filter("Image", &["png"]).save_file();
                if let (Some(output), Some(path)) = (&self.output, desired_save_path) {
                    output.save(path.as_path()).expect("Could not save image!");
                }
            }
        }
    }


    pub fn view(&self) -> Element<'_, Message> {
        // Path Selection
        let paths = [
            PathSelection::Linear,
            PathSelection::Radial,
            PathSelection::Blocks,
        ];
        let path_select_list = pick_list(paths, self.sort_path, Message::SortPathSelected);
        

        // Image Display
        let image_display = {
            if let Some(sorted_image) = &self.output {
                image(Handle::from_rgba(sorted_image.width(), sorted_image.height(), pixelsort::image::DynamicImage::ImageRgb8(sorted_image.clone()).to_rgba8().into_raw()))
            } else if let Ok(input_image) = pixelsort::open(&self.file_path) {
                image(Handle::from_rgba(input_image.width(), input_image.height(), input_image.to_rgba8().into_raw()))
            } else {
                image("imgs/landscape.png")
            }
        };


        // Interface
        column![
            row![path_select_list, button("Sort Image").on_press(Message::RunSorting), button("Save Image").on_press(Message::SaveFile)].spacing(50),
            image_display
        ].spacing(10).into()
    }
}
