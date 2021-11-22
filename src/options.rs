fn div(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {None}
    else { Some(x/y)}
}

pub fn run() {
    let res1 = div(12.0, 7.0);
    let res2 = div(12.0, 0.0);

    match res1 {
        Some(x) => println!("{:.4}", x),
        None => println!("Cannot divide by 0"),
    }
    match res2 {
        Some(x) => println!("{:.4}", x),
        None => println!("Cannot divide by 0"),
    }
}