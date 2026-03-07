use std::usize;

fn main() {
    let str1: String = String::from("Amit");
    let len = get_length(&str1);
    println!("Length of the string is: {}", len);
    println!("The string is: {}", &str1);

    // can have multiple immutable references to the same string
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    let s3 = &mut s1; // cant create multiple mutable references
    let s4 = &mut s1;
    println!(" s2: {}", s2);

}
// 1. giving ownership of the string to the function get_length, so we can not use str1 after calling the function, because it has been moved to str2 in the function get_length.
// 2. we can return the string back to the main function, so we can use it after calling the function get_length.
fn get_length(str: &String) -> usize{
    return str.len();
}

// ownership rules
// Borrowing rules
// 1. You can have only one mutable reference to a particular piece of data in a scope. This prevents data races at compile time.
// 2. You can have any number of immutable references to a particular piece of data in a scope, but you cannot have a mutable reference to that data while you have immutable references. This prevents data races at compile time.
// 3. References must always be valid. This means that you cannot have a reference to data that has been dropped. This prevents dangling references at compile time.
