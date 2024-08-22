use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use crate::astar::Position;

#[derive(Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
    pub magnitude: f32
}

impl From<[usize; 2]> for Vector {
    fn from(array: [usize; 2]) -> Self {
        Vector::component(array[0] as f32, array[1] as f32)
    }
}

impl From<Position> for Vector {
    fn from(position: Position) -> Self {
        Vector::component(position.x as f32, position.y as f32)
    }
}

impl Vector {
    pub fn new() -> Self {
        Self { x: 0.0f32, y: 0.0f32, direction: 0.0f32, magnitude: 0.0f32 }
    }

    pub fn component(x: f32, y: f32) -> Self {
        let mut vector: Self = Self::new();
        vector.x = x;
        vector.y = y;
        vector.update_polar();
        vector
    }

    pub fn polar(direction: f32, magnitude: f32) -> Self {
        let mut vector: Self = Self::new();
        vector.direction = direction;
        vector.magnitude = magnitude;
        vector.update_component();
        vector
    }

    pub fn pretty_print(&self) {
        if self.y > 0f32 {
            println!("{}i + {}j", self.x, self.y);
        } else if self.y < 0f32 {
            println!("{}i - {}j", self.x, self.y.abs());
        } else {
            println!("{}i", self.x);
        }
    }
        
    pub fn update_polar(&mut self) {
        self.magnitude = (self.x.powf(2.0f32) + self.y.powf(2.0f32)).sqrt();
        self.direction = self.y.atan2(self.x);
    }

    pub fn update_component(&mut self) {
        self.x = self.direction.cos() * self.magnitude;
        self.y = self.direction.sin() * self.magnitude;
    }

    pub fn normalize(&mut self) {
        self.x /= self.magnitude;
        self.y /= self.magnitude;
        self.magnitude = 1.0f32;
    }

    pub fn normalized(self) -> Self {
        Self::polar(self.direction, 1.0f32)
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::component(self.x + other.x, self.y + other.y)
    }
}

impl Add<&Vector> for Vector {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self::component(self.x + other.x, self.y + other.y)
    }
}

impl<'a> Add<&Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector::component(self.x + other.x, self.y + other.y)
    }
}

impl<'a> Sub<&Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector::component(self.x +- other.x, self.y - other.y)
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.update_polar();
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.update_polar();
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::component(self.x - other.x, self.y - other.y)
    }
}

impl Sub<&Vector> for Vector {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        Self::component(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.update_polar();
    }
}

impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.update_polar();
    }
}

impl Add<f32> for Vector {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self::component(self.x + other, self.y + other)
    }
}

impl AddAssign<f32> for Vector {
    fn add_assign(&mut self, other: f32) {
        self.x += other;
        self.y += other;
        self.update_polar();
    }
}

impl Sub<f32> for Vector {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self::component(self.x - other, self.y - other)
    }
}

impl SubAssign<f32> for Vector {
    fn sub_assign(&mut self, other: f32) {
        self.x -= other;
        self.y -= other;
        self.update_polar();
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self::component(self.x * other, self.y * other)
    }
}

impl<'a> Mul<f32> for &'a Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector::component(self.x * other, self.y * other)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.update_polar();
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self::component(self.x / other, self.y / other)
    }
}

impl<'a> Div<f32> for &'a Vector {
    type Output = Vector;

    fn div(self, other: f32) -> Vector {
        Vector::component(self.x / other, self.y / other)
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.update_polar();
    }
}