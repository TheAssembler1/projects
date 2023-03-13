use std::ops::Add;
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(" "))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("w = {}", w);
}

fn main() {
    println!("Hello, world!");
}
