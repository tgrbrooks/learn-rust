// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: Point {
            x: right_edge,
            y: bottom_edge,
        },
    } = rect;
    let area = (right_edge - left_edge) * (top_edge - bottom_edge);
    return area.abs();
}

fn square(point: Point, wh: f32) -> Rectangle {
    let Point { x, y } = point;
    return Rectangle {
        top_left: Point { x, y },
        bottom_right: Point {
            x: x + wh,
            y: y - wh,
        },
    };
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 5.0 },
        bottom_right: Point { x: 5.0, y: 0.0 },
    };
    println!("Rectangle area = {}", rect_area(rect));
    let p = Point { x: 0.0, y: 5.0 };
    let sq = square(p, 5.0);
    println!("{:?}", sq);
    println!("Rectangle area = {}", rect_area(sq));
}
