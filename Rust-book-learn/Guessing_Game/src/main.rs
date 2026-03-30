use std::io;
use rand::RngExt;
// use rand::thread_rng;
use rand::rng;
use rand::Rng;

fn main() {
    println!("Guess the number!: ");

    // let mut rng = rng();
    // let secret_number = rng.random_range(1..=100); // generate random number

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess:");

    let mut guess = String::new(); // variables are immutable by default

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}", guess);

    println!("Generate secret number: {}", secret_number);

}


// fn main(){
//     let mut rng = rng();
//     let a: f64 = rng.random();
//     let b: f64 = rng.random();

//     println!("{}, {}", a, b);
// }