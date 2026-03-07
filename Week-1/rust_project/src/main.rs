// entry point function
fn main() {
    let ans = is_even(4);
    println!("{}", ans);

    let name: String  = String:: from("Amit Kumar");
    // println!("{}", name); // name is valid here : ownership is with main function

    // let name2: Vec<i32> = vec![1,2,3,4,5];
    // println!("{:?}", name2);

    let (len, name): (usize, String) = get_len(name);
    println!("{}", len); // name is moved to get_len function and is no longer valid here
    println!("{}", name); 

    // let len_name2 = len_name; 
    // println!("{}", len_name2); // len_name is moved to len_name2 and is no longer valid here

    // println!("{}", len_name); // error: value borrowed here after move

}

fn is_even(a: u32) -> bool{
    return a % 2 == 0;
}

// Transferring the ownership of the variable to the function
fn get_len(s:String) -> (usize, String){
    return (s.len(), s); // returning the ownership of the variable back to the caller
}

