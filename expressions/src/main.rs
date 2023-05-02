#![allow(unreachable_code)]

fn outer_loop() {
    'outer:loop {
        println!("Entered the outer loop");

        'inner:loop {
            println!("Enterded the inner loop");

            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

fn return_loop_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("{} and {}", result, counter)
}

fn fizz_buzz() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0  {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

fn fizz_buzz_with_for() {
    for n in 1..=101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0  {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn iter_for() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name  {
            &"Ferris" => println!("There is a rustacean among us!"),
            _=> println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}

fn into_iter_for() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name  {
            "Ferris" => println!("There is a rustacean among us!"),
            _=> println!("Hello {}", name),
        }
    }
}

fn iter_mut_for() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name  {
            &mut "Ferris" => "There is a rustcean among us!",
            _=> "Hello",
        }
    }

    println!("names: {:?}", names);
}

fn match_value(number: i32) {
    println!("Tell me about {}", number);

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _=> println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

fn is_even(number: i32) {
    if number % 2 == 0 {
        println!("Is a even number: {}", number);
    } else {
        println!("Is a odd number: {}", number);
    }
}

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is small number, ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);

    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");
            break;
        }
    }

    outer_loop();

    return_loop_value();

    fizz_buzz();

    fizz_buzz_with_for();

    iter_for();

    into_iter_for();

    iter_mut_for();

    match_value(11);

    is_even(32);
    is_even(13);
    is_even(1000);
}

