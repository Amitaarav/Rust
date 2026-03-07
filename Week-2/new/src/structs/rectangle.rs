struct Rect{
    height: f32,
    width: f32
}

// writing member function use argument &self
impl Rect {
    fn area(&self)-> f32{
        return self.width * self.height;
    }

    fn print_something(){
        println!("Static function")
    }

    fn perimeter(&self)-> f32{
        return 2.0 * (self.width + self.height);
    }
}

fn main(){
    let r = Rect{
        width: 10.0,
        height: 10.0
    };

    println!("{}, {}", r.width, r.height);
    println!("{}", r.area());
    Rect::print_something();
    println!("{}", r.perimeter());
} 


