pub fn run() {
  trait Shape {
    fn area(&self) -> f32;
  }

  struct Rectangle {
    length: f32,
    width: f32,
  }
  struct Circle {
    radius: f32,
  }

  impl Shape for Rectangle {
    fn area(&self) -> f32 {
      return self.length * self.width;
    }
  }

  impl Shape for Circle {
    fn area(&self) -> f32 {
      return 3.1415 * self.radius * self.radius;
    }
  }

  // Now we can use that trait as a guarantee and common interface to call area function on shapes
  let rect = Rectangle {
    length: 5.0,
    width: 5.0,
  };
  let circ = Circle { radius: 5.0 };

  let shape1 = &rect as &dyn Shape;
  let shape2 = &circ as &dyn Shape;

  println!(
    "Rectangle area: {} \nCircle Area: {}",
    shape1.area(),
    shape2.area()
  );
}
