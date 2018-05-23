use std::io;

fn fib(number: i32) -> i32 {

    if number <= 1 {
        number
    } else {
        let x: i32 = fib(number - 1);
        let y: i32 = fib(number - 2);
        x + y
    }

}

fn main() {

    let mut input = String::new();

    println!("\nEnter an integer between 1-100:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed.");

    let max: i32 = input
        .trim()
        .parse()
        .unwrap();

    println!("\nResult: \n{}\n", fib(max));

}