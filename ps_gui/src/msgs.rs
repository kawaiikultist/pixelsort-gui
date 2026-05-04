use pixelsort::prelude::{SortKey, SortPath};

#[derive(Clone, Copy)]
pub enum Message {
    File(FileMessage),
    Param(ParamMessage),
    Sort(SortMessage),
}


#[derive(Clone, Copy)]
pub enum FileMessage {
    Save,
}

#[derive(Clone, Copy)]
pub enum ParamMessage {
    AngleChanged(f64),
    SortKeyChanged(SortKey),
    SortPathChanged(SortPath),
    ReverseToggled(bool),
    // ThresholdChanged(ThresholdMessage),
}


#[derive(Clone, Copy)]
pub enum SortMessage {
    Run,
}
