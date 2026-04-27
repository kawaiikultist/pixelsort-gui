
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
#[derive(Clone, Copy)]
pub enum SortKey {
    Luma, Red, Green, Blue
}




// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                  Sort Path                                  │
// └─────────────────────────────────────────────────────────────────────────────┘
#[derive(Clone, Copy)]
pub enum SortPath {
    Linear,
    Radial { x_offset: u32, y_offset: u32 },
    Blocks { x_size: u32, y_size: u32 },
}

impl SortPath {
    pub(crate) fn get_path(&self) -> Box<dyn crate::Path> {
        match self {
            Self::Linear => { Box::new(crate::paths::LinearPath::new()) },
            Self::Radial { x_offset, y_offset } => { Box::new(crate::paths::RadialPath::new(*x_offset, *y_offset)) },
            Self::Blocks { x_size, y_size } => { Box::new(crate::paths::BlockPath::new(*x_size, *y_size)) },
        }
    }
}




// ┌─────────────────────────────────────────────────────────────────────────────┐
// │                                Indexed Pixel                                │
// └─────────────────────────────────────────────────────────────────────────────┘
pub(crate) struct IndexedPixel {
    pub(crate) position: [u32; 2],
    pub(crate) color: image::Rgb<u8>,
}
