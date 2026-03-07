enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32)
}

// write a function that takes a Shape as an input and prints its area
// write one that prints its perimeter as well

impl Shape{
    fn area(&self) -> f32{
        return match self{
                Shape::Square(side) => side * side,
                Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
                Shape::Rectangle(length, breadth) => length * breadth
        }
    }
}
fn calculate_area(shape: Shape) -> f32{
    match shape{
        Shape::Square(side) => side * side,
        Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
        Shape::Rectangle(length, breadth) => length * breadth
    }
}

fn calculate_perimeter(shape: Shape)-> f32{
    match shape{
        Shape::Square(side) => 4.0 * side,
        Shape::Circle(radius)=> std::f32::consts::PI * 2.0 * radius,
        Shape::Rectangle(length, breadth)=> 2.0 * (length + breadth)
    }
}

fn main(){
    let square = Shape::Square(10.0);
    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(10.0, 10.0);

    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of Square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));

    let square = Shape::Square(10.0);
    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(10.0, 10.0);

    println!("Perimeter of circle: {}", calculate_perimeter(circle));
    println!("Perimeter of Square: {}", calculate_perimeter(square));
    println!("Perimeter of rectangle: {}", calculate_perimeter(rectangle));
}

// Billion dollar mistake => js null
// let a = null