struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User{
        username: String::from("Amit"),
        email: String::from("amit@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let _name = user1.username;

    user1.username = String::from("Amit Kumar");

    println!("Username: {}, Email: {}, Sign in count: {}, Active: {}", user1.username, user1.email, user1.sign_in_count, user1.active);

    let _user2 = build_user(String::from("user2@example.com"), String::from("User2"));

    let user3 = User{
        email: String::from(""),
        username: String::from(""),
        ..user1
    };

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

}

fn build_user(email: String, username: String) -> User{
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}