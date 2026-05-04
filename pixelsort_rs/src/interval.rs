use crate::types::{IndexedPixel, SortKey, PixelColor};

pub(crate) struct Interval {
    positions: Vec<[u32; 2]>,
    colors: Vec<PixelColor>,
}


impl Interval {
    pub fn new_empty() -> Self {
        Self {
            positions: Vec::new(),
            colors: Vec::new(),
        }
    }


    pub(crate) fn is_empty(&self) -> bool {
        assert_eq!(self.colors.len(), self.positions.len());
        self.colors.len() == 0
    }


    pub(crate) fn push(&mut self, pos: [u32; 2], pixel: PixelColor) {
        self.positions.push(pos);
        self.colors.push(pixel);
    }


    pub(crate) fn get_last_position(&self) -> Option<&[u32; 2]> {
        self.positions.last()
    }


    pub(crate) fn sort(&mut self, key: SortKey, reverse: bool) {
        self.colors.sort_unstable_by_key(|px| px.get_key_value(key));

        // Interval is actually reversed by default because the lowest key is first.
        if !reverse { self.colors.reverse(); }
    }


    pub(crate) fn get_indexed_pixels(&mut self) -> Vec<IndexedPixel> {
        assert_eq!(self.positions.len(), self.colors.len());

        let mut pixels: Vec<IndexedPixel> = Vec::new();
        for i in 0..self.colors.len() {
            pixels.push(IndexedPixel {
                position: self.positions[i],
                color: self.colors[i].0,
            });
        }

        pixels
    }
}
