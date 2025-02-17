use std::io;
use std::process;

fn main() {
    loop {

        println!("Please enter a first number: ");
        let a = read_user_input();

        println!("Please enter a second number: ");
        let b = read_user_input();

        let result = sum(a, b);
        println!("{} + {} = {}", a, b, result);
    }
}

fn sum(a: u32, b: u32) -> u32 {
    a+b
}

fn read_user_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let digit:u32;

    // Handle error using Pattern Matching Expression
    match input.trim().parse() {
        Ok(val) => digit = val,
        Err(_err) => {
            println!("This is not a valid number or it is too large!");
            process::exit(1);
        }
    };

    digit
}