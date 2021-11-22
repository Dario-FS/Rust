// Structs - Used to create custom data types

// Traditional Struct
// struct Color {
//   red: u8,
//   green: u8,
//   blue: u8,
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
  name: String,
  age: u8,
}
#[payable]
#[private]
impl Person {
  // Construct person
  fn new(first: &str, last: u8) -> Person {
    Person {
      name: first.to_string(),
      age: last
    }
  }

  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.name, self.age)
  }

  // Set age
  fn set_age(&mut self, last: u8) {
    self.age = last
  }

  // Data to tuple
  fn to_tuple(self) -> (String, u8) {
    (self.name, self.age)
  }
}

pub fn run() {
  // let mut c = Color {
  //   red: 255,
  //   green: 0,
  //   blue: 0,
  // };

  // c.red = 200;

  // println!("Color: {} {} {}", c.red, c.green, c.blue);

  // let mut c = Color(255, 0, 0);

  // c.0 = 200;

  // println!("Color: {} {} {}", c.0, c.1, c.2);

  let mut p = Person::new("Mary", 18);
  println!("Person {}", p.full_name());
  p.set_age(19);
  println!("Person {}", p.full_name());
  println!("Person Tuple {:?}", p.to_tuple());
}
