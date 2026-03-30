fn main(){
    let reference_to_nothing = dangle(); // This would cause a compile-time error, because we are trying to return a reference to a value that is created inside the function and will be dropped when the function ends. We cannot return a reference to a value that will go out of scope.
    println!("{}", reference_to_nothing);
}

fn dangle() -> &String{
    let s = String::from("hello dangle");
    //s // s is returned and moves out to the calling function. This is valid, because we are returning the ownership of the string to the calling function, not a reference to it.
    &s
}


// Rules of References:
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid. This means that the value that a reference points to must not go out of scope while the reference is still in use. In the dangle function, we are trying to return a reference to a value that is created inside the function and will be dropped when the function ends, which is why it causes a compile-time error.