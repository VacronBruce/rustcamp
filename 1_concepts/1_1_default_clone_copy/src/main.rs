use std::default;

use smart_default::SmartDefault;

#[derive(Clone, Copy, SmartDefault)]
struct Point {
    #[default = 100.0]
    x: f64,
    #[default = 2000.0]
    y: f64,
}
#[derive(Clone)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn new(points: Vec<Point>) -> Self {
        Self {
            points
        }
    }

    fn add(&mut self, item: Point) {
        self.points.push(item)
    }

    fn get(&self, i: usize) -> Option<&Point> {
        self.points.get(i)
    }

    fn remove(&mut self, i: usize) -> Option<Point> {
        if i >= self.points.len() {
            return None;
        }
        return Some(self.points.remove(i))
    }
}


fn main() {
    println!("Hello, world!");
}
