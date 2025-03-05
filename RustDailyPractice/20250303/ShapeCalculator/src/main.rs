// Exercise 2: Shape Calculator
// Create a system to calculate area and perimeter of different shapes:
//
// Implement a Shape enum that can represent:
//
// Circle (radius)
// Rectangle (width, height)
// Triangle (three sides)
// Square (side length)
//
//
// Implement methods on the enum to:
//
// Calculate the area of any shape
// Calculate the perimeter of any shape
// Determine if a shape is regular (all sides equal)

const PI: f64 = 3.141592653589793;

enum Shape {
    Circle {
        radius: f64,
    },
    Rectangle {
        width: f64,
        height: f64,
    },
    Triangle {
        points: [(f64, f64); 3],
    },
    Square {
        size: f64,
    },
}

fn calc_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle { points } => {
            0.5 * ((points[0].0 * (points[1].1 - points[2].1))
                + (points[1].0 * (points[2].1 - points[0].1))
                + (points[2].0 * (points[0].1 - points[1].1)))
                .abs()
        },
        Shape::Square { size } => size * size,
    }
}

// Helper function to calculate the distance between two points
fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt()
}

fn calc_perimeter(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => 2.0 * PI * radius,
        Shape::Rectangle { width, height } => 2.0 * (width + height),
        Shape::Triangle { points } => {
            let a = distance(points[0], points[1]);
            let b = distance(points[1], points[2]);
            let c = distance(points[2], points[0]);
            a + b + c
        }
        Shape::Square { size } => 4.0 * size,
    }
}


fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle {
            points: [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)],
        },
        Shape::Square { size: 4.0 },
    ];

    for (i, shape) in shapes.iter().enumerate() {
        let area = calc_area(shape);
        let perimeter = calc_perimeter(shape);
        println!(
            "Shape {} - Area: {:.2}, Perimeter: {:.2}",
            i + 1,
            area,
            perimeter
        );
    }
}