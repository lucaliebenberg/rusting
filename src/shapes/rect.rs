use std::fmt::Display;
use crate::shapes::collisions::Collidable;
use crate::Circle;

use super:: area::Area;
use super::collisions::{Contains, Points};

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Contains for Rect {
    fn contains_point(&self, (x,y): (f64, f64)) -> bool {
        return 
            self.x <= x && self.x + self.width >= x && 
            self.y <= y && self.y + self.height >= y; 
    }
}

impl Points for Rect {
    fn points(&self) -> super::collisions::PointIter {
        todo!()
    }
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rect {
    fn default() -> Self {
        return Rect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f, 
            "Rectangle({}, {}): {}x{}",
         self.x,
         self.y,
         self.width,
         self.height
        );
    }
}

pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64); 

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        return self.points.get(idx)
            .map(|x| * x);
    }
}

impl From<&Rect> for RectIter {
    fn from(value: &Rect) -> Self {
        return RectIter {
            points:
            idx: 0
        };
    }
}

impl IntoIterator for Rect {
    type Item = (f64, f64); 

    type IntoIterator = RectIter;

    fn into_iter(self) -> Self::IntoIterator {
        return RectIter {
            points: vec![
                (self.x, self.y),
                (self.x, self.width, self.y),
                (self.x, self.y, self.height),
                (self.x + self.width, self.y + self.height),               
            ],
            idx: 0,
        };
    }

}