fn main() {
    let original = String::from("Hello");
    let moved = original; // ownership transferred to 'moved'

    // Erro: 'original' no longer accessible
    // println!("{}", original);

    let mut str = String::from("Amit");
    let ref1 = &mut str;
    ref1.push_str("Kumar");
    let ref2 = &str;
    println!("{}", ref2);
}
