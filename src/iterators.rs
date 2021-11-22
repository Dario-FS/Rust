trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// 2
fn is_even(n: u32) -> bool {
    n % 2 == 0
}

pub fn run() {
    // 1
    let v = vec![1, 2, 3];
    v.iter().next();
    
    // 2
    let top = 1000;
    let mut c = 0;
    for i in 0.. {
        let x = i*i;
        if x >= top { break}
        else if is_even(x) { c += x }
    }
    println!("{}", c);

    //equivalent but using closures
    let s: u32 = 
    (0..).map(|n| n*n)
    .take_while(|&n| n < 1000)
    .filter(|&n| is_even(n))
    .fold(0, |s,i| s + i);

    println!("{}", s)
}