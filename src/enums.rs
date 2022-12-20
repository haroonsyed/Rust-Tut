enum Movement {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

fn move_avatar(m: Movement) {
  match m {
    Movement::UP => println!("Avatar moving: UP"),
    Movement::DOWN => println!("Avatar moving: DOWN"),
    Movement::LEFT => println!("Avatar moving: LEFT"),
    Movement::RIGHT => println!("Avatar moving: RIGHT"),
  }
}

pub fn run() {
  move_avatar(Movement::UP);
}
