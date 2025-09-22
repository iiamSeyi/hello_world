//Structs to represent geometric shapes
struct Rectangle {
    width: f64,
    height: f64,
}
struct Circle {
    radius: f64, // assume radius > 0
}

// Implement methods for Structs
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        // TODO: implement
        self.width * self.height// Calculates the area
    }

    fn perimeter(&self) -> f64 {
        // TODO: implement
        2.0 * (self.width + self.height)// Calculates the perimeter
    }

    fn is_square(&self) -> bool {
        // TODO: implement
        self.width == self.height // Checks if width and height are equal
    }
}

impl Circle {
    //Creates a new Circle instance with the given radius
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    //Calculates the area of the circle
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }

    //Calculates the circumference of the circle
    fn circumference(&self) -> f64 {
        2.0 * 3.14 * self.radius
    }
}

fn main() {
    let rect = Rectangle::new(10.0, 5.0);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square? {}", rect.is_square());

    let circle = Circle::new(7.0);
    println!("Area: {}", circle.area());
    println!("Circumference: {}", circle.circumference());

    assert!(Rectangle::new(5.0, 5.0).is_square());
    assert!(!Rectangle::new(5.0, 6.0).is_square());

    println!("All tests passed!"); // Wont run if assertions fail
}
