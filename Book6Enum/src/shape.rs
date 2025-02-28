enum Shape {
    Circle(f64),                       // Radius
    Rectangle(f64, f64),               // Width, Height
    Triangle(f64, f64, f64),           // Side A, Side B, Side C
    Square(f64),                       // Side length
}

impl Shape {
    // Method to calculate area that works on all variants
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            },
            Shape::Square(side) => side * side,
        }
    }

    // Method to calculate perimeter
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle(width, height) => 2.0 * (width + height),
            Shape::Triangle(a, b, c) => a + b + c,
            Shape::Square(side) => 4.0 * side,
        }
    }

    // Constructor method for creating a square (static method)
    fn new_square(side: f64) -> Shape {
        Shape::Square(side)
    }

    // Method to check if the shape is valid
    fn is_valid(&self) -> bool {
        match self {
            Shape::Circle(radius) => *radius > 0.0,
            Shape::Rectangle(width, height) => *width > 0.0 && *height > 0.0,
            Shape::Triangle(a, b, c) => {
                // Triangle inequality theorem
                *a > 0.0 && *b > 0.0 && *c > 0.0 &&
                    a + b > *c && a + c > *b && b + c > *a
            },
            Shape::Square(side) => *side > 0.0,
        }
    }

    // Method that returns a new shape scaled by a factor
    fn scale(&self, factor: f64) -> Shape {
        if factor <= 0.0 {
            panic!("Scale factor must be positive");
        }

        match self {
            Shape::Circle(radius) => Shape::Circle(radius * factor),
            Shape::Rectangle(width, height) => Shape::Rectangle(width * factor, height * factor),
            Shape::Triangle(a, b, c) => Shape::Triangle(a * factor, b * factor, c * factor),
            Shape::Square(side) => Shape::Square(side * factor),
        }
    }

    // Method that returns a description of the shape
    fn describe(&self) -> String {
        match self {
            Shape::Circle(radius) => format!("Circle with radius {:.2}", radius),
            Shape::Rectangle(width, height) => format!("Rectangle with width {:.2} and height {:.2}", width, height),
            Shape::Triangle(a, b, c) => format!("Triangle with sides {:.2}, {:.2}, and {:.2}", a, b, c),
            Shape::Square(side) => format!("Square with side length {:.2}", side),
        }
    }
}

fn main() {
    // Create different shapes
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 7.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);
    let square = Shape::new_square(6.0);  // Using the constructor

    // Calculate and print areas
    println!("Circle area: {:.2}", circle.area());
    println!("Rectangle area: {:.2}", rectangle.area());
    println!("Triangle area: {:.2}", triangle.area());
    println!("Square area: {:.2}", square.area());

    // Check validity
    let invalid_triangle = Shape::Triangle(1.0, 1.0, 10.0);  // Violates triangle inequality
    println!("Is the triangle valid? {}", invalid_triangle.is_valid());

    // Scale a shape
    let big_circle = circle.scale(2.0);
    println!("{}", big_circle.describe());
    println!("Original circle area: {:.2}", circle.area());
    println!("Scaled circle area: {:.2}", big_circle.area());

    // Demonstrate how to work with a collection of shapes
    let shapes = vec![circle, rectangle, triangle, square];
    let total_area: f64 = shapes.iter().map(|shape| shape.area()).sum();
    println!("Total area of all shapes: {:.2}", total_area);
}