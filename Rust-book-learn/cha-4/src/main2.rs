fn main(){
    let mut s = String::from("hello ownership");
    let r1 = & s;
    let r2 = & s;
    println!("r1: {}, r2: {}", r1, r2); // This is valid, because we are only reading from s, not modifying it. We can have multiple immutable references to a value.

    let r3 = & mut s; 
    println!("r3: {}", r3); 
}