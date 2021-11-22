use std::ops::Mul;

struct S<T> {
    x: T,
    y: T,
}

impl <T> S<T> {
    fn item(&self) -> &T {
        &self.x
    }
}

// 2

trait Mult<T> {
    fn area(&self) -> T;
}

impl <T: Copy> Mult<T> for S<T> where T: Mul<Output = T>, {
    fn area(&self) -> T {
        self.x * self.y
    }
}

pub fn run(){
    let a = S{x: 6, y: 9};
    let b = S{x: 3.0, y: 6.0};
    let c = S{x: "Hi", y: "there"};

    //item only return the same value
    a.item();

    println!("{} {} {}", a.x, b.x, c.y);

    // 2
    let d = S{x: 3, y: 9};
    println!("{}", d.area())
}

