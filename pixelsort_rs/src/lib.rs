pub mod utils;
pub mod types;
mod interval;
mod paths;


use image::{ImageBuffer, Rgb};
use crate::{
    paths::{LinearPath, Path}, 
    types::{SortKey, SortPath}
};


// Re-Exports
pub use image::open;
pub use utils::get_test_image;


// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                 Pixelsorter                                 │
// └─────────────────────────────────────────────────────────────────────────────┘
pub struct Pixelsorter {
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    angle: f64,
    key: SortKey,
    path: Box<dyn Path>,
    reverse: bool,
}


impl Pixelsorter {
    pub fn new(image: ImageBuffer<Rgb<u8>, Vec<u8>>, angle: f64, key: SortKey, path_type: SortPath, reverse: bool) -> Self {
        Self {
            image,
            angle,
            key,
            path: path_type.get_path(),
            reverse,
        }
    }


    pub fn new_linear(image: ImageBuffer<Rgb<u8>, Vec<u8>>, angle: f64, reverse: bool) -> Self {
        Self {
            image,
            angle,
            key: SortKey::Luma,
            path: Box::new(LinearPath::new()),
            reverse,
        }
    }

    pub fn get_sorted_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut intervals = self.path.get_intervals(&self.image, self.angle);

        let mut output = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.image.width(), self.image.height());
 
        for intvl in &mut intervals {
            intvl.sort(self.key, self.reverse);

            for px in intvl.get_indexed_pixels() {
                output.put_pixel(px.position[0], px.position[1], px.color);
            }
        }

        output
    }
}
