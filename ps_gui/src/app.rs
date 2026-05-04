use std::{path::PathBuf};

use iced::{
    Application, Element, widget::{Image, button, column, combo_box, image::{self, Handle}, pick_list, row, slider, text}
};
use pixelsort::{image::DynamicImage, prelude::*};

use crate::msgs::{FileMessage, Message, ParamMessage, SortMessage};


#[derive(Default)]
pub struct App {
    input_image: DynamicImage,

    sort_angle: f64,
    sort_key: SortKey,
    sort_path: SortPath,
    sort_reverse: bool,

    output_image: Option<DynamicImage>,
}


impl App {
    pub fn new(input_image: DynamicImage) -> Self {
        Self { input_image, ..Default::default() }
    }


    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::File(m) => self.handle_file_message(m),
            Message::Param(m) => self.handle_param_message(m),
            Message::Sort(m) => self.handle_sort_message(m),
        }
    }


    pub fn view(&self) -> Element<'_, Message> {
        row![
            column![
                // Angle Slider
                row![text(format!("Angle: {}", self.sort_angle)), slider(0.0..=360.0, self.sort_angle, |v| Message::Param(ParamMessage::AngleChanged(v)))].spacing(10),

            ],
            // TODO: Remove this.
            button("Sort & Save").on_press(Message::Sort(SortMessage::Run)),
        ].into()
    }


    fn handle_file_message(&mut self, msg: FileMessage) {
    }

    fn handle_param_message(&mut self, msg: ParamMessage) {
        match msg {
            ParamMessage::AngleChanged(a) => self.sort_angle = a,
            ParamMessage::SortKeyChanged(k) => self.sort_key = k,
            ParamMessage::SortPathChanged(p) => self.sort_path = p,
            ParamMessage::ReverseToggled(r) => self.sort_reverse = r,
        }
    }

    fn handle_sort_message(&mut self, msg: SortMessage) {
        match msg {
            SortMessage::Run => {
                self.output_image = Some(Pixelsorter::new(self.input_image.clone())
                    .set_angle(self.sort_angle.to_radians())
                    .set_path(self.sort_path)
                    .set_key(self.sort_key)
                    .set_reverse(self.sort_reverse)
                    .run());

                if let Some(image) = &self.output_image {
                    image.save("imgs/gui_angle.png").expect("Couldn't save image!");
                }
            }
        }
    }
}


// #[derive(Clone, Copy, Debug, Default, PartialEq)]
// enum PathSelection {
//     #[default]
//     Linear,
//     Radial,
//     Blocks,
// }
//
// impl PathSelection {
//     fn get_sort_path(&self) -> SortPath {
//         match self {
//             Self::Linear => { SortPath::Linear },
//             Self::Radial => { SortPath::Radial { x_offset: 0, y_offset: 0 }},
//             Self::Blocks => {SortPath::Blocks { x_size: 64, y_size: 64 }},
//         }
//     }
// }
//
//
// impl std::fmt::Display for PathSelection {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(match self {
//             Self::Linear => "Linear",
//             Self::Radial => "Radial",
//             Self::Blocks => "Blocks",
//         })
//     }
// }
//
//
// #[derive(Clone, Debug)]
// pub enum Message {
//     SortPathSelected(PathSelection),
//     RunSorting,
//     SaveFile,
// }
//
//
// #[derive(Default)]
// pub struct App {
//     pub file_path: PathBuf,
//
//     sort_path: Option<PathSelection>,
//
//     output: Option<RgbImage>
// }
//
//
// impl App {
//     pub fn new(file_path: PathBuf) -> Self {
//         Self { file_path, ..Default::default()}
//     }
//
//     pub fn update(&mut self, msg: Message) {
//         match msg {
//             Message::SortPathSelected(ps) => { self.sort_path = Some(ps); },
//             Message::RunSorting => {
//                 let image = pixelsort::open(self.file_path.as_path())
//                     .expect("Could not open image!").to_rgb8();
//
//                 let path = {
//                     if let Some(selection) = self.sort_path {
//                         selection.get_sort_path()
//                     } else {
//                         SortPath::Linear
//                     }
//                 };
//
//                 self.output = Some(Pixelsorter::new(image)
//                     .set_path(path)
//                     .set_key(SortKey::Luma)
//                     .run());
//             },
//             Message::SaveFile => {
//                 let desired_save_path = FileDialog::new().add_filter("Image", &["png"]).save_file();
//                 if let (Some(output), Some(path)) = (&self.output, desired_save_path) {
//                     output.save(path.as_path()).expect("Could not save image!");
//                 }
//             }
//         }
//     }
//
//
//     pub fn view(&self) -> Element<'_, Message> {
//         // Path Selection
//         let paths = [
//             PathSelection::Linear,
//             PathSelection::Radial,
//             PathSelection::Blocks,
//         ];
//         let path_select_list = pick_list(paths, self.sort_path, Message::SortPathSelected);
//
//
//         // Image Display
//         let image_display = {
//             if let Some(sorted_image) = &self.output {
//                 image(Handle::from_rgba(sorted_image.width(), sorted_image.height(), pixelsort::image::DynamicImage::ImageRgb8(sorted_image.clone()).to_rgba8().into_raw()))
//             } else if let Ok(input_image) = pixelsort::open(&self.file_path) {
//                 image(Handle::from_rgba(input_image.width(), input_image.height(), input_image.to_rgba8().into_raw()))
//             } else {
//                 image("imgs/landscape.png")
//             }
//         };
//
//
//         // Interface
//         column![
//             row![path_select_list, button("Sort Image").on_press(Message::RunSorting), button("Save Image").on_press(Message::SaveFile)].spacing(50),
//             image_display
//         ].spacing(10).into()
//     }
// }
