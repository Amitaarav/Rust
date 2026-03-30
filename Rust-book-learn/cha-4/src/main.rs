fn main() {
    let x = 5;
    let y = x; // x is copied to y, because i32 implements the Copy trait. Both x and y are valid after this line.

    println!("x: {}, y: {}", x, y); // x and y are valid here : ownership is with main function

    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 is cloned to s2, because String implements the Clone trait. Both s1 and s2 are valid after this line.
    println!("s1: {}", s1); // This is valid
    println!("s2: {}", s2); // This is also valid

    let s3 = String::from("world");
    take_ownership(s3); // s3 is moved to the function, and is not valid after this line. Ownership is transferred to the function.
    //println!("s3: {}", s3); // This would cause a compile-time error
    make_copy(x); // x is copied to the function, and is still valid after this line. Ownership is not transferred to the function.
    println!("x: {}", x); // This is valid

    let s4 = gives_ownership(); // s4 takes ownership of the string returned by gives_ownership
    println!("s4: {}", s4); // This is valid


    let s5 = String::from("hello takes and gives back");
    let s6 = takes_and_gives_back(s5); // s5 is moved to the function, and s6 takes ownership of the string returned by takes_and_gives_back
    //println!("s5: {}", s5); // This would cause a compile-time error
    println!("s6: {}", s6); // This is valid

    let mut s7 = String::from("hello calculate length");
    let len = calculate_length(&s7); // s7 is moved to the function, and s8 takes ownership of the string returned by calculate_length. len is the length of the string.
    //println!("s7: {}", s7); // This would cause a compile-time error
    println!("length: {}", len);

    change(&mut s7); // This would cause a compile-time error, because s7 is an immutable reference. We cannot modify it.

    println!("s7: {}", s7); // This is valid, because we are only reading from s7, not modifying it.
}   

fn take_ownership(some_string: String){
    println!("{}", some_string);
}

fn make_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String{
    let some_string = String::from("hello givership");
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize{
    // reference are immutable by default, so we cannot modify s in this function. We can only read from it. We can also return a value that is not a reference, because we are not returning a reference to s.

    s.len() // len() returns the length of a String
}

fn change(some_string: &mut String){
    some_string.push_str(", world"); // This would cause a compile-time error, because some_string is an immutable reference. We cannot modify it.
}

