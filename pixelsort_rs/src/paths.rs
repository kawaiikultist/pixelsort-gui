use image::{
    ImageBuffer,
    Rgb,
};

use crate::{
    interval::Interval,
    types::Vec2,
};

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                 Path Trait                                  │
// └─────────────────────────────────────────────────────────────────────────────┘
pub(crate) trait Path {
    fn get_intervals(&self, image: &ImageBuffer<Rgb<u8>, Vec<u8>>, angle: f64) -> Vec<Interval>;

    // TODO: Padding for sine sorting?
    fn get_edge_pixels(&self, image_width: u32, image_height: u32, direction: Option<&Vec2>) -> Vec<[u32; 2]> {
        let mut starts: Vec<[u32; 2]> = Vec::new();

        for x in 0..image_width {
            for y in 0..image_height {
                // If the pixel is on the border
                if x == 0 || x == image_width - 1 || y == 0 || y == image_height - 1 {
                    if let Some(dir) = direction {

                        // If a direction is supplied get only the relevant border pixels.
                        let (prev_x, prev_y) = (x as f64 - dir.x, y as f64 - dir.y);
                        if prev_x < 0.0 || prev_x > image_width as f64 || prev_y < 0.0 || prev_y > image_width as f64 {
                            starts.push([x, y]);
                        }

                    } else {
                        // Otherwise get all border pixels.
                        starts.push([x, y]);
                    }
                }
            }
        }

        starts
    }

}


// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                 Linear Path                                 │
// └─────────────────────────────────────────────────────────────────────────────┘
pub(crate) struct LinearPath;

impl LinearPath {
    pub(crate) fn new() -> Self { Self }
}

impl Path for LinearPath {
    fn get_intervals(&self, image: &ImageBuffer<Rgb<u8>, Vec<u8>>, angle: f64) -> Vec<Interval> {
        let mut intervals: Vec<Interval> = Vec::new();
        let direction = Vec2::new_from_angle(angle);
        let starts = self.get_edge_pixels(image.width(), image.height(), Some(&direction));

        for start_pos in starts {
            let mut intvl = Interval::new_empty();
            let mut px = Vec2::new(start_pos[0] as f64, start_pos[1] as f64);

            while px == px.clamped(Vec2::ZERO, Vec2::new(image.width() as f64 - 1.0, image.height() as f64 - 1.0)) {
                // Get pixel coordinates
                let pxi: [u32; 2] = [px.x.round() as u32, px.y.round() as u32];

                // Move px in direction
                px += direction;

                // Check if current position is last position
                if intvl.get_last_position() == Some(&pxi) { continue; }

                // Add the pixel to intvl
                intvl.push(pxi, *image.get_pixel(pxi[0], pxi[1]));
            }
            
            // Add intvl to intervals
            intervals.push(intvl);
        }

        intervals
    }
}
