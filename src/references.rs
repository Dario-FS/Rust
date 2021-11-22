fn take(v: Vec<u32>) -> Vec<u32> {
    println!("{}", v[1] + v[2]);
    v
}

fn borrow1(v: &Vec<u32>) {
    println!("{}", (*v)[1] + (*v)[2]);
}

fn borrow2(v: &Vec<u32>) {
    println!("{}", &v[1] + &v[2]);
}

pub fn run() {
    let mut v  = Vec::new();

    for i in 1..=5 {
        v.push(i)
    }
    v = take(v);
    borrow1(&v);
    borrow2(&v);
}