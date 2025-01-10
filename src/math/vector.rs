use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

pub trait AboutEq {
    fn about_eq(self, v: Self) -> bool;
    fn about_zero(self) -> bool;
}

pub type Number = f64;
pub const EPSILON: Number = 1e-9;

impl AboutEq for Number {
    fn about_eq(self, v: Self) -> bool {
        (v - self).abs() <= EPSILON
    }
    fn about_zero(self) -> bool {
        self.abs() <= EPSILON
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: Number,
    pub y: Number,
}

impl Vector {
    pub const ZERO: Vector = Vector { x: 0.0, y: 0.0 };

    pub const POSY: Vector = Vector { x: 0.0, y: 1.0 };
    pub const NEGY: Vector = Vector { x: 0.0, y: -1.0 };
    pub const POSX: Vector = Vector { x: 1.0, y: 0.0 };
    pub const NEGX: Vector = Vector { x: -1.0, y: 0.0 };
    
    pub const POSINF: Vector = Vector { x: Number::INFINITY, y: Number::INFINITY };
    pub const NEGINF: Vector = Vector { x: Number::NEG_INFINITY, y: Number::NEG_INFINITY };

    pub fn mag(self) -> Number {
        Number::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn dist(self, rhs: Self) -> Number {
        (rhs - self).mag()
    }
    pub fn unit(self) -> Vector {
        let d = self.mag();
        self / d
    }
    pub fn unit_mag(self) -> (Vector, Number) {
        let d = self.mag();
        (self / d, d)
    }
    pub fn dot(self, rhs: Self) -> Number {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn cross(self, rhs: Self) -> Number {
        self.x * rhs.y - self.y * rhs.x
    }
    pub fn perp(self) -> Vector {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn rot(self, angle: Number) -> Vector {
        let v = Vector::from_angle(angle);
        Vector {
            x: self.x*v.x-self.y*v.y,
            y: self.x*v.y+self.y*v.x
        }
    }
    pub fn from_angle(angle: Number) -> Vector {
        Vector {
            x: Number::cos(angle),
            y: Number::sin(angle)
        }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

pub fn bounding_box<'a, I>(vectors: I) -> (Vector, Vector)
where
    I: IntoIterator<Item = &'a Vector>,
{
    let mut min = Vector::POSINF;
    let mut max = Vector::NEGINF;
    for v in vectors.into_iter() {
        if v.x < min.x { min.x = v.x }
        if v.y < min.y { min.y = v.y }
        if v.x > max.x { max.x = v.x }
        if v.y > max.y { max.y = v.y }
    }
    (min, max)
}

impl AboutEq for Vector {
    fn about_eq(self, v: Self) -> bool {
        self.x.about_eq(v.x) && self.y.about_eq(v.y)
    }

    fn about_zero(self) -> bool {
        self.x.about_zero() && self.y.about_zero()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Number> for Vector {
    type Output = Vector;

    fn add(self, rhs: Number) -> Self::Output {
        Vector {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<Number> for Vector {
    fn add_assign(&mut self, rhs: Number) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<Number> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Number) -> Self::Output {
        Vector {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign<Number> for Vector {
    fn sub_assign(&mut self, rhs: Number) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<Number> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Number) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<Number> for Vector {
    fn mul_assign(&mut self, rhs: Number) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Vector {
    type Output = Vector;

    fn div(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<Number> for Vector {
    type Output = Vector;

    fn div(self, rhs: Number) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<Number> for Vector {
    fn div_assign(&mut self, rhs: Number) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
