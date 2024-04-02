use std::io;

fn main() {
    println!("Welcome to brix-calculator, which will we be doing?");
    println!("1. Brix -> Alcohol by Volume");
    println!("2. Alcohol by Volume -> Brix");

    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("Please enter 1 or 2");

    let operation: u32 = operation.trim().parse().expect("Please type a number!");

    match operation {
        1 => ,
    }
}

