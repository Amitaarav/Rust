#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width * self.height > other.width * other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}
fn main(){
    // let width1 = 30;
    // let height1 = 50;

    // let rect = (30, 50);

    let rect = Rectangle{
        width: 40,
        height: 50
    };

    let rect2 = Rectangle{
        width: 20,
        height: 30
    };

    let rect3 = Rectangle{
        width: 50,
        height: 60
    };

    println!("{:#?}", rect);
    
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    // println!("The area of the rectangle is {} square pixels.", area(&rect));

    println!("The area of the rectangle is {} square pixels.", rect.area());

    let rect4 = Rectangle::square(20);
    println!("The area of the square is {} square pixels.", rect4.area());
}

// fn area(rectangle: &Rectangle) -> u32{
//     // dimensions.0 * dimensions.1
//     rectangle.width * rectangle.height
// }