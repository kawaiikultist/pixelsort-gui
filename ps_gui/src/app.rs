use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, center, column, container, image, row, text};
use iced::{
    ContentFit, Element, Length, widget
};
use pixelsort::image::DynamicImage;
use pixelsort::prelude::{Pixelsorter, SortKey, SortPath, Threshold};
use rfd::FileDialog;

use crate::{widgets, styles};
use crate::msgs::*;
use crate::utils;

// ──────────────────────────────── Panes ────────────────────────────────
enum PaneContent {
    SortingControls,
    ImageViewer,
}


// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                     App                                     │
// └─────────────────────────────────────────────────────────────────────────────┘
pub struct App {
    // TODO: Maybe store the original image and allow an undo

    // Input
    image: DynamicImage,

    // Internal Doodads.
    panes: widget::pane_grid::State<PaneContent>,
    image_handle: image::Handle,

    // Sorting Parameters
    sort_angle: f32,
    sort_key: SortKey,
    sort_path: SortPath, // FIXME: NOT IMPLEMENTED
    // Path
    // Threshold
    // Reverse

    // Output
    result: DynamicImage,
}


impl App {
    pub const THEME: iced::Theme = iced::Theme::CatppuccinMacchiato;

    pub fn new() -> Self {
    
        let (mut state, pane) = widget::pane_grid::State::new(PaneContent::SortingControls);
        state.split(widget::pane_grid::Axis::Vertical, pane, PaneContent::ImageViewer).unwrap();

        let input_image = pixelsort::utils::get_noise_image(512, 512);
        let result = input_image.clone();
        let image_handle = utils::get_image_handle(&input_image);

        Self {
            image: input_image,

            panes: state,
            image_handle,

            sort_angle: 0.0,
            sort_key: SortKey::Luma,
            sort_path: SortPath::Linear,

            result,
        }

    }


    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::DoNothing => { println!("Doing Nothing!") },

            Message::Interface(m) => { self.handle_interface_messages(m) },
            Message::File(m) => { self.handle_file_messages(m) },
            Message::Param(m) => { self.handle_param_messages(m) },
            Message::Sort(m) => { self.handle_sort_messages(m) },
        }
    }


    pub fn view(&self) -> Element<'_, Message> {

        widget::pane_grid(&self.panes, |_pane, state, _is_maximized| {
            widget::pane_grid::Content::new(
                match state {
                    PaneContent::SortingControls => self.get_sort_control_content(),
                    PaneContent::ImageViewer => self.get_image_viewer_content(),
                }
            )
        })
        .on_resize(10, |resize| Message::Interface(InterfaceMessage::PaneResized(resize)))
        .into()

    }

    // ───────────────────────── Interface Messages ──────────────────────
    fn handle_interface_messages(&mut self, msg: InterfaceMessage) {
        let InterfaceMessage::PaneResized(resize) = msg;
        self.panes.resize(resize.split, resize.ratio);
    }


    // ─────────────────────────── Param Messages ────────────────────────
    fn handle_param_messages(&mut self, msg: ParamMessage) {
        match msg {
            ParamMessage::AngleChanged(v) => self.sort_angle = v,
            ParamMessage::SortKeyChanged(v) => self.sort_key = v,
            ParamMessage::SortPathChanged(v) => self.sort_path = v,
            _ => (),
        }
    }

    // ──────────────────────────── Sort Messages ────────────────────────────
    fn handle_sort_messages(&mut self, msg: SortMessage) {
        // INFO: Currently the only SortMessage is Run.
        self.result = Pixelsorter::new(self.image.clone())
            .set_angle(self.sort_angle.into())
            .set_key(self.sort_key)
            .set_path(self.sort_path)
            .set_threshold(Threshold::new(0.0, 1.0, SortKey::Luma, false))
            .set_reverse(false)
            .run();
        
        self.image_handle = utils::get_image_handle(&self.result);
    }

    // ──────────────────────────── File Messages ────────────────────────────
    fn handle_file_messages(&mut self, msg: FileMessage) {
        match msg {
            FileMessage::Save => {
                if let Some(mut save_path) = FileDialog::new().add_filter("PNG Image", &["png"]).save_file() {
                    save_path.set_extension("png");
                    self.result.save(save_path).expect("Could not save result!");
                }
            },

            FileMessage::Load => {
                if let Some(load_path) = FileDialog::new().add_filter("Image", &["png", "jpg", "jpeg"]).pick_file() {
                    self.image = pixelsort::open(load_path).expect("Could not load image!");
                    self.image_handle = utils::get_image_handle(&self.image);
                }
            }
        }
    }

    // ┌─────────────────────────────────────────────────────────────────────────────┐
    // │                                Pane Content                                 │
    // └─────────────────────────────────────────────────────────────────────────────┘
    fn get_sort_control_content(&self) -> Element<'_, Message> {
        container(
            column![
                widgets::param_slider("Angle", 0.0, 360.0, self.sort_angle, |v| Message::Param(ParamMessage::AngleChanged(v))),
                widgets::sort_key_pick_list(self),
                widgets::sort_path(self),

                // Bottom Stuff
                container(
                    column![
                        button(text("Sort").align_x(Horizontal::Center).width(Length::Fill)).on_press(Message::Sort(SortMessage::Run)),
                        row![
                            button(text("Save").align_x(Horizontal::Center)).on_press(Message::File(FileMessage::Save)).width(Length::Fill),
                            button(text("Load").align_x(Horizontal::Center)).on_press(Message::File(FileMessage::Load)).width(Length::Fill),
                        ].spacing(10)
                    ].spacing(10)
                ).align_bottom(Length::Fill)
            ].spacing(20.0)
        )
        .padding(25.0).style(|_| styles::sort_control_pane(&App::THEME)).into()
    }

    
    fn get_image_viewer_content(&self) -> Element<'_, Message> {
        container(
            image(&self.image_handle).content_fit(ContentFit::Contain)
        )
        .padding(25.0)
        .center(Length::Fill)
        .style(|_| styles::image_viewer_pane(&App::THEME))
        .into()
    }


    // ┌─────────────────────────────────────────────────────────────────────────────┐
    // │                                   Getters                                   │
    // └─────────────────────────────────────────────────────────────────────────────┘
    pub fn get_sort_key(&self) -> SortKey { self.sort_key }
    pub fn get_sort_path(&self) -> SortPath { self.sort_path }
    pub fn get_image_size(&self) -> [u32; 2] { [self.image.width(), self.image.height()] }
}
