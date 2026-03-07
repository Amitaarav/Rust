use std::fs;

enum Result{
    Ok(String),
    Err(String)
}

enum Shape{
    Circle(f32),
    Square(f32)
}

// Option enum
enum Option1{
    Some(u32),
    None
}

fn find_first_a(str: String) -> Option<u32>{
    let mut index = 0;
    for c in str.chars(){
        if c == 'a'{
            return Some(index)
        }
    index = index + 1;
    }
    None
}

fn main(){
    let contents = fs::read_to_string("a.txt");

    match contents {
        Ok(contents) => println!("{}", contents),
        Err(_)=> println!("Error while reading file")
    } 

    let ans = find_first_a(String::from("amit gupta"));

    match ans {
        None => print!("Value not found"),
        Some(value) => print!("a found at index {}", value)
    }
}

fn get_shape() -> Shape{
    return Shape::Square(10.0);
}