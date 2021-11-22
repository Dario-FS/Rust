// 2
fn create() -> Box<dyn Fn()> {
    Box::new(move || println!("This is a clousure in a box"))
}

pub fn run() {
    // 1
    let a = |i| i + 1;
    let b = 2;
    println!("{}", a(b));

    let c = || println!("Is a clousure without argument");
    c();

    // 2
    let d = create();
    d();
}