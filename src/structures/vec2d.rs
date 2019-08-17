#![allow(dead_code)]
use rand::{thread_rng, Rng};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use std::{cmp, fmt};

// extern crate cgmath;

pub fn lerp(a: Vec2d, b: Vec2d, d: f64) -> Vec2d {
    a + (b - a).normalized() * d
}

pub fn random_in_unit_sphere() -> Vec2d {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec2d::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
        if p.length_squared() < 1. {
            return p;
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn x_comp(self) -> Vec2d {
        Vec2d::new(self.x, 0.)
    }

    pub fn y_comp(self) -> Vec2d {
        Vec2d::new(0., self.y)
    }

    pub fn new(x: impl Scalar, y: impl Scalar) -> Vec2d {
        Vec2d {
            x: x.float(),
            y: y.float(),
        }
    }

    pub fn eucl(x: impl Scalar, y: impl Scalar) -> f64 {
        let x = x.float();
        let y = y.float();
        (x * x + y * y).sqrt()
    }

    pub fn length(&self) -> f64 {
        Vec2d::eucl(self.x, self.y)
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn zero() -> Vec2d {
        Vec2d::new(0., 0.)
    }

    pub fn one() -> Vec2d {
        Vec2d::new(1., 1.)
    }

    pub fn scale(&self, scalar: f64) -> Vec2d {
        Vec2d::new(self.x * scalar, self.y * scalar)
    }

    pub fn lerp(v1: Vec2d, v2: Vec2d, alpha: f64) -> Vec2d {
        v1 + (v2 - v1) * alpha
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vec2d {
        Vec2d::new(self.x.max(min).min(max), self.y.max(min).min(max))
    }

    pub fn distance(&self, v: Vec2d) -> f64 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Generates a random Vec2d across a uniform distribution using the answer found in
    /// http://stackoverflow.com/questions/5408276/python-uniform-spherical-distribution
    pub fn random() -> Vec2d {
        let mut rng = thread_rng();
        let phi: f64 = rng.gen_range(0.0, 2.0 * ::std::f64::consts::PI);
        let costheta: f64 = rng.gen_range(-1.0, 1.0);
        let u: f64 = rng.gen_range(0.0, 1.0);

        let theta = costheta.acos();
        let r = u.powf(1.0 / 3.0);

        Vec2d::new(r * theta.sin() * phi.cos(), r * theta.sin() * phi.sin())
    }

    pub fn normalize(&mut self) {
        let norm = Vec2d::eucl(self.x, self.y);
        self.x /= norm;
        self.y /= norm;
    }

    pub fn normalized(&self) -> Vec2d {
        let scale = 1. / self.length();
        Vec2d::new(self.x * scale, self.y * scale)
    }

    pub fn dot(&self, vec: Vec2d) -> f64 {
        self.x * vec.x + self.y * vec.y
    }

    pub fn cross(self, vec: Vec2d) -> f64 {
        self.x * vec.y - self.y * vec.x
    }

    fn vec_from_angle(angle: f64) -> Vec2d {
        let vx = angle.sin();
        let vy = angle.cos();
        Vec2d::new(vx, vy)
    }

    pub fn angle(self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn recip(self) -> Vec2d {
        Vec2d::new(self.x.recip(), self.y.recip())
    }

    pub fn min(self, other: impl Into<Vec2d>) -> Vec2d {
        let other = other.into();
        Vec2d::new(self.x.min(other.x), self.y.min(other.y))
    }

    ///Get a Vec2d with the maximum of each component of this and another Vec2d
    pub fn max(self, other: impl Into<Vec2d>) -> Vec2d {
        let other = other.into();
        Vec2d::new(self.x.max(other.x), self.y.max(other.y))
    }

    pub fn as_slice(&self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, other: Vec2d) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<f64> for Vec2d {
    type Output = Vec2d;

    fn add(self, other: f64) -> Vec2d {
        Vec2d::new(self.x + other, self.y + other)
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<f64> for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: f64) -> Vec2d {
        Vec2d::new(self.x - other, self.y - other)
    }
}

impl Mul for Vec2d {
    type Output = Vec2d;

    fn mul(self, other: Vec2d) -> Vec2d {
        Vec2d::new(self.x * other.x, self.y * other.y)
    }
}

impl Mul<f64> for Vec2d {
    type Output = Vec2d;

    fn mul(self, other: f64) -> Vec2d {
        Vec2d::new(self.x * other, self.y * other)
    }
}

impl Div for Vec2d {
    type Output = Vec2d;

    fn div(self, other: Vec2d) -> Vec2d {
        Vec2d::new(self.x / other.x, self.y / other.y)
    }
}

impl Div<f64> for Vec2d {
    type Output = Vec2d;

    fn div(self, other: f64) -> Vec2d {
        Vec2d::new(self.x / other, self.y / other)
    }
}

impl Neg for Vec2d {
    type Output = Vec2d;

    fn neg(self) -> Vec2d {
        Vec2d::new(-self.x, -self.y)
    }
}

impl cmp::PartialEq for Vec2d {
    fn eq(&self, other: &Vec2d) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Debug for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Scalar, U: Scalar> From<(T, U)> for Vec2d {
    fn from(other: (T, U)) -> Vec2d {
        Vec2d::new(other.0, other.1)
    }
}

impl<T: Scalar> From<[T; 2]> for Vec2d {
    fn from(other: [T; 2]) -> Vec2d {
        Vec2d::new(other[0], other[1])
    }
}

// impl<T: Scalar> From<cgmath::Point2<T>> for Vec2d {
//     fn from(other: cgmath::Point2<T>) -> Vec2d {
//         Vec2d::new2d(other[0], other[1])
//     }
// }

// impl<T: Scalar> From<cgmath::Point3<T>> for Vec2d {
//     fn from(other: cgmath::Point3<T>) -> Vec2d {
//         Vec2d::new(other[0], other[1], other[2])
//     }
// }

// impl<T: Scalar+std::fmt::Debug+std::cmp::PartialEq> From<nalgebra::geometry::Point2<T>> for Vec2d {
//     fn from(other: nalgebra::geometry::Point3<T>) -> Vec2d {
//         Vec2d::new2d(other[0], other[1])
//     }
// }

// impl<T: Scalar> From<nalgebra::geometry::Point3<T>> for Vec2d {
//     fn from(other: nalgebra::geometry::Point3<T>) -> Vec2d {
//         Vec2d::new(other[0], other[1], other[2])
//     }
// }

pub trait Scalar: Copy {
    fn float(self) -> f64;
}

impl Scalar for u8 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for u16 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for u32 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for i8 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for i16 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}
impl Scalar for i32 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}

impl Scalar for f32 {
    fn float(self) -> f64 {
        f64::from(self)
    }
}

impl Scalar for f64 {
    fn float(self) -> f64 {
        self
    }
}
