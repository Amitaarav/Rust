fn main() {
    let mut x = 5;

    println!("The value of x is: {}", x);

    let x: &str = "six"; // shadow

    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000; // can not mutate constant
    // type annotated, have to annotate

    // shadowing:allows to create new variable using an existing name

}
