// extern crate rand;
pub mod vec2d;
pub mod vec3d;

pub use vec2d::Vec2d;
pub use vec3d::Vec3d;

use rstar::Point;

#[derive(Copy, Clone)]
pub struct IndexValue3d {
    pub vec: Vec3d,
    pub index: usize,
}

impl std::fmt::Debug for IndexValue3d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{index}: ({}, {}, {})",
            self.vec.x,
            self.vec.y,
            self.vec.z,
            index = self.index
        )
    }
}

impl std::cmp::PartialEq for IndexValue3d {
    fn eq(&self, other: &IndexValue3d) -> bool {
        self.vec.x == other.vec.x && self.vec.y == other.vec.y && self.vec.z == other.vec.z
    }
}

impl Point for IndexValue3d {
    type Scalar = f64;
    const DIMENSIONS: usize = 3;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        IndexValue3d {
            vec: Vec3d::new(generator(0), generator(1), generator(2)),
            index: 0,
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.vec.x,
            1 => self.vec.y,
            2 => self.vec.z,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.vec.x,
            1 => &mut self.vec.y,
            2 => &mut self.vec.z,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct IndexValue2d {
    pub vec: Vec2d,
    pub index: usize,
}

impl std::fmt::Debug for IndexValue2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{index}: ({}, {})",
            self.vec.x,
            self.vec.y,
            index = self.index
        )
    }
}

impl std::cmp::PartialEq for IndexValue2d {
    fn eq(&self, other: &IndexValue2d) -> bool {
        self.vec.x == other.vec.x && self.vec.y == other.vec.y
    }
}

impl Point for IndexValue2d {
    type Scalar = f64;
    const DIMENSIONS: usize = 2;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        IndexValue2d {
            vec: Vec2d::new(generator(0), generator(1)),
            index: 0,
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.vec.x,
            1 => self.vec.y,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.vec.x,
            1 => &mut self.vec.y,
            _ => unreachable!(),
        }
    }
}
