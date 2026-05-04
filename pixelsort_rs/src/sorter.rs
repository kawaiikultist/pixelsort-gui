
use image::{DynamicImage, ImageBuffer, Rgba};
use crate::{
    paths::Path,
    types::{SortKey, SortPath, Threshold}
};

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                 Pixelsorter                                 │
// └─────────────────────────────────────────────────────────────────────────────┘
pub struct Pixelsorter {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    angle: f64,
    key: SortKey,
    path: Box<dyn Path>,
    reverse: bool,
    threshold: Threshold,
}


impl Pixelsorter {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            image: image.to_rgba8(),
            angle: 0.0,
            key: SortKey::Luma,
            path: Box::new(crate::paths::LinearPath::new()),
            reverse: false,
            threshold: Threshold::new(0.0, 1.0, SortKey::Luma, false),
        }
    }


    pub fn set_angle(mut self, angle: f64) -> Self {
        self.angle = angle;
        self
    }


    pub fn set_key(mut self, key: SortKey) -> Self {
        self.key = key;
        self
    }


    pub fn set_path(mut self, path: SortPath) -> Self {
        self.path = path.get_path();
        self
    }


    pub fn set_reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }


    pub fn set_threshold(mut self, threshold: Threshold) -> Self {
        self.threshold = threshold;
        self
    }


    pub fn run(&self) -> DynamicImage {//-> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut intervals = self.path.get_intervals(&self.image, self.angle, self.threshold);
        let mut output = self.image.clone();

        for intvl in &mut intervals {
            intvl.sort(self.key, self.reverse);
            
            for px in intvl.get_indexed_pixels() {
                output.put_pixel(px.position[0], px.position[1], px.color);
            }
        }

        DynamicImage::ImageRgba8(output)

        // output
    } 
    // pub fn new(image: ImageBuffer<Rgba<u8>, Vec<u8>>, angle: f64, key: SortKey, path_type: SortPath, reverse: bool, threshold: Threshold) -> Self {
    //     Self {
    //         image,
    //         angle,
    //         key,
    //         path: path_type.get_path(),
    //         reverse,
    //         threshold,
    //     }
    // }
    //
    //
    // pub fn get_sorted_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    //     let mut intervals = self.path.get_intervals(&self.image, self.angle, self.threshold);
    //
    //     let mut output = self.image.clone();
    //
    //     for intvl in &mut intervals {
    //         intvl.sort(self.key, self.reverse);
    //
    //         for px in intvl.get_indexed_pixels() {
    //             output.put_pixel(px.position[0], px.position[1], px.color);
    //         }
    //     }
    //
    //     output
    // }
}
