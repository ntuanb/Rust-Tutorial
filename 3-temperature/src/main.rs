use std::io;
use std::process;

fn main() {

    let mut temperature_type = String::new();
    let mut temperature = String::new();

    println!("\nChoose temperature type to convert to C or F:");

    io::stdin()
        .read_line(&mut temperature_type)
        .expect("Failed type.");

    let temperature_type = temperature_type.trim();

    match temperature_type {

        "C" => {
            println!("");
        },
        "F" => {
            println!("");
        },
        _ => {
            println!("Incorrect type entered.");
        }

    };

    println!("Please enter the temperature:");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed temperature.");

    let mut result: f32 = temperature
        .trim()
        .parse()
        .unwrap();


    if temperature_type == "F" {
        result = result * 1.8 + 32.0;
    } else {
        result = result - 32.0 / 1.8;
    }

    println!("\nThe converted temperature is {}{}.\n", result, temperature_type);

    process::exit(1);

    // println!("Your temperature entered is: {}", input);

}