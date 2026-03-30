// recap traits

// interfaces vs traits
/**
 *  1. Interfaces can be implemented by classes
 */
use std::f32::consts::PI;
 trait Shape{
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
 }

 struct Rect {
    width: f32,
    height: f32
 }

 struct Circle {
   radius: f32
 }

 impl Shape for Rect{
    fn area(&self) -> f32{
        return self.width * self.height;
    }

    fn perimeter(&self) -> f32{
         return 2.0 * (self.width + self.height);
    }
 }

 impl Shape for Circle{
   fn area(&self) -> f32{
      return PI * self.radius * self.radius;
   }

   fn perimeter(&self) -> f32{
      return 2.0 * PI * self.radius;
   }
 }

 fn get_area_a(s: &impl Shape) -> f32{
    return s.area();
 }

 fn get_perimeter(s: impl Shape) -> f32{
   return s.perimeter();
 }
fn main() {

    let r = Rect {
        width: 10.0,
        height: 20.0
    };

    let c = Circle {
      radius: 5.0
    };

    let rect_area = get_area_a(&r);
    println!("Area of rectangle: {}", rect_area);

    let rect_perimeter = get_perimeter(r);
    println!("Perimeter of rectangle: {}", rect_perimeter);

    let circle_area = get_area_a(&c);
    println!("Area of circle: {}", circle_area);
}
