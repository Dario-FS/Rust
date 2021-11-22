// 1
fn f<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() == y.len() { x }
    else { y }
}

// 2
struct A<'a, 'b> {
    x: &'a str,
    y: &'b str,
}

pub fn run() {
    // 1
    let a = "string a";
    let b = "string b";

    let c = f(a,b);

    println!("{}", c);

    // 2
    let d = A{x: "Hi", y: "bro"};

    // 3
    let e: &'static str = "entire lifetime";
}