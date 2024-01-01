use crate::types::{Vec3, Point3};


pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn origin(self: &Self) -> &Point3 { &self.origin }
    pub fn direction(self: &Self) -> &Vec3 { &self.direction }
    pub fn at(self: &Self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
