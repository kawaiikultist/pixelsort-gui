use image::{Pixel, Rgb};
use crate::types::{IndexedPixel, SortKey};

pub(crate) struct Interval {
    positions: Vec<[u32; 2]>,
    colors: Vec<Rgb<u8>>,
}


impl Interval {
    pub fn new_empty() -> Self {
        Self {
            positions: Vec::new(),
            colors: Vec::new(),
        }
    }


    pub(crate) fn push(&mut self, pos: [u32; 2], pixel: Rgb<u8>) {
        self.positions.push(pos);
        self.colors.push(pixel);
    }


    pub(crate) fn get_last_position(&self) -> Option<&[u32; 2]> {
        self.positions.last()
    }


    pub(crate) fn sort(&mut self, key: SortKey, reverse: bool) {
        self.colors.sort_unstable_by_key(|px| {
            match key {
                SortKey::Luma => { px.to_luma()[0] },
                SortKey::Red => { px[0] },
                SortKey::Green => { px[1] },
                SortKey::Blue => { px[2] },
            }
        });

        // Interval is actually reversed by default because the lowest key is first.
        if !reverse { self.colors.reverse(); }
    }


    pub(crate) fn reverse(&mut self) {
        self.positions.reverse();
    }


    pub(crate) fn get_indexed_pixels(&mut self) -> Vec<IndexedPixel> {
        assert_eq!(self.positions.len(), self.colors.len());

        let mut pixels: Vec<IndexedPixel> = Vec::new();
        for i in 0..self.colors.len() {
            pixels.push(IndexedPixel {
                position: self.positions[i],
                color: self.colors[i],
            });
        }

        pixels
    }
}
