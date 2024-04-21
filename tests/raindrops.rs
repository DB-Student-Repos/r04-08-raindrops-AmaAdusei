fn raindrops(n: i32) -> String {
    let mut result = String::new();

    if n % 3 == 0 {
        result.push_str("Pling");
    }
    if n % 5 == 0 {
        result.push_str("Plang");
    }
    if n % 7 == 0 {
        result.push_str("Plong");
    }

    if result.is_empty() {
        result = n.to_string();
    }

    result
}

use std::io;

fn main() {
    println!("Enter a number:");

    let mut input_num = String::new();

    io::stdin().read_line(&mut input_num)
        .expect("Failed to read line");
    let num: i32 = input_num.trim().parse()
        .expect("Please enter a valid number");

    println!("{}", raindrops(num)); 
}


