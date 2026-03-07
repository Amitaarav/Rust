use rectangle::Rect;
pub mod rectangle;

fn main(){
    let mut r = Rect{
        width: 10.0,
        height: 10.0
    };

    r.width = 100.0;
    r.height = 100.0;

    println!("{}, {}", r.width, r.height);
    print!("{}", r.area());
}