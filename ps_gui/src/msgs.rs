use pixelsort::prelude::{SortKey, SortPath};

#[derive(Clone, Copy)]
pub enum Message {
    DoNothing,

    Interface(InterfaceMessage),
    File(FileMessage),
    Param(ParamMessage),
    Sort(SortMessage),
}


#[derive(Clone, Copy)]
pub enum FileMessage {
    Load,
    Save,
}

#[derive(Clone, Copy)]
pub enum ParamMessage {
    AngleChanged(f32),
    SortKeyChanged(SortKey),
    SortPathChanged(SortPath),
    ReverseToggled(bool),
    // ThresholdChanged(ThresholdMessage),
}


#[derive(Clone, Copy)]
pub enum SortMessage {
    Run,
}


#[derive(Clone, Copy)]
pub enum InterfaceMessage {
    PaneResized(iced::widget::pane_grid::ResizeEvent),
}
