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

impl Debug for User{
    fn fmt(&self. f: &mut fmt::Formatter) -> Result{
        return 
    }
}
fn main(){
    let u = User{
        username: String::from("Amit"),
        password: String::from("amit"),
        age: 32
    };

    println!("{}", u.username);
    // print!("{:?}", u);
}