fn main() {
    let s = "hello";
    // string literals are stored in the binary and are immutable, so they have a static lifetime
    println!("{}", s); // s is valid here : ownership is with main function

    let mut s2 = String::from("Hello Amit!");
    s2.push_str(", how are you?"); // error: cannot borrow `s` as mutable, as it is not declared as mutable
    println!("{}", s2); // s is valid here : ownership is with main function


    let s3 = String::from("Hello Amit!");
    let s4 = s3; // s3 is moved to s4 and is no
    // longer valid here
    println!("{}", s4); // s4 is valid here : ownership is with main function
    println!("{}", s3); // error: value borrowed here after move
}
// s remains until it goes out of scope
// String can be mutated but literals not

// difference between string literals and String type
// how therse two types deal with memory

//

