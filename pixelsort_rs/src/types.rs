use image::Pixel;


// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                    Vec2                                     │
// └─────────────────────────────────────────────────────────────────────────────┘
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}


impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }


    pub fn new_from_angle(rad: f64) -> Self {
        Self {
            x: f64::cos(rad),
            y: f64::sin(rad)
        }
    }


    pub fn normalize_or_zero(self) -> Self {
        let len = (self.x * self.x) + (self.y * self.y);

        if len < 0.01 {
            Self::new(0.0, 0.0)
        } else {
            Self::new(
                self.x / len,
                self.y / len
            )
        }
    }


    pub fn clamped(self, min: Vec2, max: Vec2) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }


    pub fn distance_to_squared(&self, other: Vec2) -> f64 {
        (other.x - self.x).powi(2) + (other.y - self.y).powi(2)
    }

    
    pub fn direction_to(&self, other: Vec2) -> Vec2 {
        Vec2::new(other.x - self.x, other.y - self.y).normalize_or_zero()
    }
}

// ╓                                                                             ╖
// ║                            Basic Math Operations                            ║
// ╙                                                                             ╜
impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}




// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                  Sort Key                                   │
// └─────────────────────────────────────────────────────────────────────────────┘
#[derive(Clone, Copy, Default, PartialEq)]
pub enum SortKey {
    #[default]
    Luma,
    Hue,
    Saturation,
    // Red,
    // Green,
    // Blue
}


impl SortKey {
    pub fn get_list() -> [Self; 3] {  [ Self::Luma, Self::Hue, Self::Saturation]  }
}


impl core::fmt::Display for SortKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            SortKey::Luma => "Luma",
            SortKey::Hue => "Hue",
            SortKey::Saturation => "Saturation",
        };

        write!(f, "{}", txt)
    }
}


// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                  Sort Path                                  │
// └─────────────────────────────────────────────────────────────────────────────┘
#[derive(Clone, Copy, Default, PartialEq)]
pub enum SortPath {
    #[default]
    Linear,
    Radial { x_offset: i32, y_offset: i32 },
    Blocks { x_size: u32, y_size: u32 },
}

impl SortPath {
    pub fn get_path(&self) -> Box<dyn crate::paths::Path> {
        match self {
            Self::Linear => { Box::new(crate::paths::LinearPath::new()) },
            Self::Radial { x_offset, y_offset } => { Box::new(crate::paths::RadialPath::new(*x_offset, *y_offset)) },
            Self::Blocks { x_size, y_size } => { Box::new(crate::paths::BlockPath::new(u32::max(*x_size, 1), u32::max(*y_size, 1))) },
        }
    }

    pub fn get_radial_offset(&self) -> Option<[i32; 2]> {
        if let Self::Radial { x_offset, y_offset } = self { Some([*x_offset, *y_offset]) }
        else { None }
    }

    pub fn get_block_size(&self) -> Option<[u32; 2]> {
        if let Self::Blocks { x_size, y_size } = self { Some([u32::max(*x_size, 1), u32::max(*y_size, 1)]) }
        else { None }
    }


    pub fn get_list() -> [SortPath;3] { [SortPath::Linear, SortPath::Radial { x_offset: 0, y_offset: 0 }, SortPath::Blocks { x_size: 0, y_size: 0 }] }
}

impl core::fmt::Display for SortPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Self::Linear => "Linear",
            Self::Radial { x_offset: _, y_offset: _} => "Radial",
            Self::Blocks { x_size: _, y_size: _ } => "Blocks",
        };

        write!(f, "{}", txt)
    }
}




// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                Indexed Pixel                                │
// └─────────────────────────────────────────────────────────────────────────────┘
pub(crate) struct IndexedPixel {
    pub(crate) position: [u32; 2],
    pub(crate) color: image::Rgba<u8>,
}




// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                 Pixel Color                                 │
// └─────────────────────────────────────────────────────────────────────────────┘
pub struct PixelColor(pub image::Rgba<u8>);


impl PixelColor {
    fn r(&self) -> u8 { self.0[0] }
    fn g(&self) -> u8 { self.0[1] }
    fn b(&self) -> u8 { self.0[2] }

    fn normalized_rgb(&self) -> (f32, f32, f32) {
        (
            self.r() as f32 / 255.0,
            self.g() as f32 / 255.0,
            self.b() as f32 / 255.0
        )
    }

    pub(crate) fn get_key_value(&self, key: SortKey) -> u8 {
        match key {
            SortKey::Luma => { self.0.to_luma().0[0] },
            SortKey::Hue => {
                let (r, g, b) = self.normalized_rgb();

                let max = r.max(g).max(b);
                let min = r.min(b).min(g);
                let diff = max - min;


                let hue = {
                    if max == min { 0.0 }
                    else if max == r { (60.0 * ((g - b) / diff) + 360.0) % 360.0 }
                    else if max == g { (60.0 * ((b - r) / diff) + 120.0) % 360.0 }
                    else { (60.0 * ((r - g) / diff) + 240.0) % 360.0 }
                };

                ((hue / 360.0) * 255.0) as u8
            },
            SortKey::Saturation => {
                let (r, g, b) = self.normalized_rgb();
                
                let min = r.min(g).min(b);
                let max = r.max(g).max(b);
                let saturation = (max - min) / max;

                (saturation.clamp(0.0, 1.0) * 255.0) as u8
            },
        }
    }
}



// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                  Threshold                                  │
// └─────────────────────────────────────────────────────────────────────────────┘
#[derive(Copy, Clone)]
pub struct Threshold {
    min: u8,
    max: u8,
    key: SortKey,
    invert: bool,
}


impl Threshold {
    pub fn new(min_pct: f64, max_pct: f64, key: SortKey, invert: bool) -> Self {
        Self {
            min: (min_pct * 255.0) as u8,
            max: (max_pct * 255.0) as u8,
            key, invert
        }
    }


    pub fn pixel_in_range(&self, px: &PixelColor) -> bool {
        let v = px.get_key_value(self.key);

        if self.invert { v != v.clamp(self.min, self.max) }
        else { v == v.clamp(self.min, self.max) }
    }
}
