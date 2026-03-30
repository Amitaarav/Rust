fn main(){
    let s = String::from("hello ownership");
    let word_index = first_word(&s);

    println!("Index of the first word: {}", word_index);
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    println!("Bytes: {:?}", bytes);
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()

}