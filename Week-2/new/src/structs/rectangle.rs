pub struct Rect{
    pub height: f32,
    pub width: f32
}

// writing member function use argument &self
impl Rect {
    pub fn area(&self)-> f32{
        return self.width * self.height;
    }

    pub fn print_something(){
        println!("Static function")
    }

    pub fn perimeter(&self)-> f32{
        return 2.0 * (self.width + self.height);
    }
}



