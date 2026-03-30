fn main(){
    let x = 5;
    let x = x + 1;
    //let x = x.to_string(); //Because shadowing allows type change (i32 → String)
    println!("{}", x);

    let x = x * 2;
    {
        let x = x + 10;
        println!("{}", x);
    }
    println!("{}", x);

    let s = " 100 ";
    let num = s.trim();
    let num: i32 = num.parse().unwrap();
    let num = num / 2;
    println!("{}", num);
}

// Create a new variable
// Can change type
// Old variable is discarded

