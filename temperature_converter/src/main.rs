use std::io::stdin;

fn celsius_to_fahrenheit() -> () {
    let celsius = value_choice();

    let fahrenheit = (celsius * (9.0/5.0)) + 32.0;
    println!("Converted {} celsius to {} fahrenheit", celsius, fahrenheit)
}

fn fahrenheit_to_celsius() -> () {
    let fah = value_choice();
    
    let celsius = (fah - 32.0) * 5.0 / 9.0;
    println!("Converted {} fahrenheit to {} celsius", fah, celsius)
}

fn value_choice() -> f64 {
    let response: f64;

    loop {
        println!("Insert the value to be converted");

        let mut value_to_converter = String::new();

        stdin()
            .read_line(&mut value_to_converter)
            .expect("You need to provide the value that you want to convert");

        response = match value_to_converter
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You need to provide the value that you want to convert");
                    continue;
                }
            };

        break;
    }

    return response
}

fn main() {
    println!("Insert the convertion that you want to make: ");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Other: Quit");

    loop {
        let mut choice = String::new();

        stdin()
            .read_line(&mut choice)
            .expect("You need to insert a choice");

        let choice: u8 = match choice
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => break,
            };

        match choice {
            1 => { 
                celsius_to_fahrenheit();
                break;
            },
            2 => {
                fahrenheit_to_celsius();
                break;
            },
            _ => break,
        };
            
    }
}
