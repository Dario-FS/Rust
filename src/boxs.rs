use std::fmt::Debug;

// 2
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    End,
}

use List::{Cons, End};

pub fn run() {
    // 1
    let x = 2; 
    let y = &x;
    let z = Box::new(x);

    if *y == *z {println!("True")};

    // 2
    let l = Cons(1, Box::new(Cons(2, Box::new(End))));
    println!("{:#?}", l);
}