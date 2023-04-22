use std::f64::consts::PI;

struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

fn main() {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{}", circ.area());
    println!("{}", rect.area())
}