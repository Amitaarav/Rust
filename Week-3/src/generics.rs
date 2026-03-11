use std::ops::Add;
fn sum<W: Add<Output = W>>(a: W, b: W) -> W{
    return a + b;
}

fn main(){
    println!("{}", sum(3, 5));
}
