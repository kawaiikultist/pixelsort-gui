pub mod prelude;
pub mod utils;

mod types;
mod interval;
mod paths;
mod sorter;

// Re-Exports
pub use image::{
    self,
    open,
    ImageBuffer,
    Rgb,
    RgbImage,
};



// TODO: SHOULD PROBABLY REWRITE THISE ALL TO BE BASED ON DYNAMIC IMAGE INSTEAD LMAO
