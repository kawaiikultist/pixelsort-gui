use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, center, column, container, image, row, text};
use iced::{
    ContentFit, Element, Length, widget
};
use pixelsort::image::DynamicImage;
use pixelsort::prelude::{Pixelsorter, SortKey, Threshold};
use rfd::FileDialog;

use crate::{widgets::*, styles};
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
    // Input
    image: DynamicImage,

    // Internal Doodads.
    panes: widget::pane_grid::State<PaneContent>,
    image_handle: image::Handle,

    // Sorting Parameters
    sort_angle: f32,

    // Output
    result: DynamicImage,
}


impl App {
    pub const THEME: iced::Theme = iced::Theme::CatppuccinMacchiato;

    pub fn new(input_image: DynamicImage) -> Self {
    
        let (mut state, pane) = widget::pane_grid::State::new(PaneContent::SortingControls);
        state.split(widget::pane_grid::Axis::Vertical, pane, PaneContent::ImageViewer).unwrap();

        let result = input_image.clone();
        let image_handle = utils::get_image_handle(&input_image);

        Self {
            image: input_image,

            panes: state,
            image_handle,

            sort_angle: 0.0,

            result,
        }

    }


    pub fn update(&mut self, msg: Message) {
        match msg {
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
        // if let InterfaceMessage::PaneResized( resize ) = msg {
        //     self.panes.resize(resize.split, resize.ratio);
        // }
    }


    // ─────────────────────────── Param Messages ────────────────────────
    fn handle_param_messages(&mut self, msg: ParamMessage) {
        if let ParamMessage::AngleChanged(a) = msg {
            self.sort_angle = a;
        }
        // match msg {
        //     ParamMessage::AngleChanged(a) => { self.sort_angle = a },
        //     _ => (),
        // }
    }

    // ──────────────────────────── Sort Messages ────────────────────────────
    fn handle_sort_messages(&mut self, msg: SortMessage) {
        if let SortMessage::Run = msg {
            self.result = Pixelsorter::new(self.image.clone())
                .set_angle(self.sort_angle.into())
                .set_key(SortKey::Luma)
                .set_path(pixelsort::prelude::SortPath::Linear)
                .set_threshold(Threshold::new(0.0, 1.0, SortKey::Luma, false))
                .set_reverse(false)
                .run();
            
            self.image_handle = utils::get_image_handle(&self.result);
        }
    }

    // ──────────────────────────── File Messages ────────────────────────────
    fn handle_file_messages(&mut self, msg: FileMessage) {
        // FIXME: Crashes if the user doesn't manually add '.png' to filename
        if let FileMessage::Save = msg {
            let save_path = FileDialog::new().add_filter("Png", &["png"]).save_file();

            if let Some(path) = save_path {
                self.result.save(path).expect("Could not save result!");
            }
        }
    }

    // ┌─────────────────────────────────────────────────────────────────────────────┐
    // │                                Pane Content                                 │
    // └─────────────────────────────────────────────────────────────────────────────┘
    fn get_sort_control_content(&self) -> Element<'_, Message> {
        container(
            column![
                param_slider("Angle", 0.0, 360.0, self.sort_angle, |v| Message::Param(ParamMessage::AngleChanged(v))),

                row![
                    button(text("Sort").align_x(Horizontal::Center)).on_press(Message::Sort(SortMessage::Run)).width(Length::Fill),
                    button(text("Save").align_x(Horizontal::Center)).on_press(Message::File(FileMessage::Save)).width(Length::Fill),
                ].spacing(10).height(Length::Fill).align_y(Vertical::Bottom)
            ].spacing(20.0)//.height(Length::Fill).height(Length::Fill)
        )
        // .width(Length::Fill).height(Length::Fill)
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
}
