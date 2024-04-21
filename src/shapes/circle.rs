use std::f64::consts::PI;
use super:: area::Area;
use crate::shapes::collisions::Collidable;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn contains_point(&self, (x,y): (f64, f64)) -> bool {
        let x = self.x - x;
        let y = self.y - y;

        return dx * dx + dy * d <= self.radius * self.radius;
    }
}

impl Collidable<Rect> for Circle {
    fn collide(&self, other: &Rect) -> bool {
        for point in other {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}

impl Collidable<Circle> for Circle {
    fn collide(&self, other: &Circle) -> bool {
        for point in other {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}