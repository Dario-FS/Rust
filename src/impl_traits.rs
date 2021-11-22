use std::ops;

// 1
trait Shape {
    fn area(&self) -> u32;
}

struct Circle {
    radius:f32,
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.14 * self.radius *self.radius) as u32
    }
}

// 2
struct A; struct B;
#[derive(Debug)]
struct AB;
#[derive(Debug)]
struct BA;

impl ops::Add<B> for A {
    type Output = AB;
    fn add(self, _rhs: B) -> AB { AB }
}

// 3 -DROP-
struct D {d: String,}

impl Drop for D {
    fn drop(&mut self) {
        println!("dropped {}", self.d)
    }
}

pub fn run() {
    // 1
    let c = Circle{radius: 13.69};
    println!("{}", c.area());

    // 2
    println!("{:?}", A + B);

    // 3
    let d = D{d: String::from("C")};
    drop(d);
    println!("Leaving inner scop")
}