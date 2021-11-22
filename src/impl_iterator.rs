
struct Fib{ c: u32, n:u32}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib{c: 0, n: 1}
}

pub fn run() {
    for i in fib().take(5){
        println!("{}", i);
    }
    for j in fib().skip(5).take(5){
        println!("{}", j);
    }

    //to go one by one
    let mut f = fib();
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
}