mod shapes;

use crate::shapes::{
    rect::Rect, 
    circle::Circle, 
    // area::Area
};

fn main() {
    /* excutor file */
     let rect = Rect::default();
     let rect2 = Rect::default();

    let circle = Circle {
        x: 1.2,
        y: 2.0,
        radius: 1.0,
    };

    rect.collide(&rect2);

    println!("{:?}",circle)
}