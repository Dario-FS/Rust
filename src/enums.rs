// Enums are types which have a few definite values

enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  // Perform action depending on info
  match m {
    Movement::Up => println!("Avatar moving up"),
    Movement::Down => println!("Avatar moving down"),
    Movement::Left => println!("Avatar moving left"),
    Movement::Right => println!("Avatar moving right"),
  }
}

fn matching_single(n: u8) {
  match n {
    1 => println!("is one"),
    2|3|5|7|11 => println!("is prime"),
    12..=19 => println!("between 12 and 19"),
    _ => println!("other"),
  }
}

fn matching_tuple(a: i8, b: i8) {
  match (a,b) {
    (0, y) => println!("y: {}", y),
    (x, 0) => println!("y: {}", x),
    _ => println!("No match"),
  }
}

fn matching_including(v: u8) {
  match v {
    v @ 0..=5  => println!("{} is between 0 and 5", v),
    v @ 6..=10  => println!("{} is between 6 and 10", v),
    _ => println!("other"),
  }
}

pub fn run() {
  let avatar1 = Movement::Left;
  let avatar2 = Movement::Up;
  let avatar3 = Movement::Right;
  let avatar4 = Movement::Down;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);

  let n = 9;
  matching_single(n);

  let pair: (i8, i8) = (0, -1);
  matching_tuple(pair.0, pair.1);

  matching_including(n)
}
