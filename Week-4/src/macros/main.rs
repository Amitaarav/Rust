// macro_rules! say_hello{
//     () => {
//         println!("Hello")
//     }
// }

// fn main(){
//     say_hello!();
// }


// declarative : macro_rules
// procedural : #[derive(Debug)]

#[derive(Debug)]
struct User{
    username: String,
    password: String,
    age: u32
}

fn main(){
    let u = User{
        username: String::from("Amit");
        password: String::from("amit");
        age: 32
    };

    print!("{:?}", u);
}