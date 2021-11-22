use std::collections::HashMap;

pub fn run() {
    let mut hm = HashMap::new();
    hm.insert(String::from("random"), 12);
    hm.insert(String::from("string"), 41);

    for(k, v) in &hm {
        println!("{}: {}",k ,v);
    }

    // .get allow access to a element inside the HashMap
    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("No match"),
    }

    hm.remove(&String::from("String"));
}