use let_else::let_else;
use while_let::while_let;

pub mod if_let;
pub mod let_else;
pub mod while_let;

fn arrays_slices() {
    let array = [1, -2, 6];

    match array {
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => println!("array[0] = 1, array[2] = {}, array[1] was ignored", third),
        [-1, second, ..] => println!("array[0] = -1, array[1] = {} and all the other ones were ignored", second),
        [3, second, tail @ ..] => println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        [first, middle @ .., last] => println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }
}

fn tuples() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (..,2) => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _ => println!("It doen't matter what they are"),
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,

    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn enums() {
    // let color = Color::RGB(122, 17, 40);
    let color = Color::Red;

    println!("what color is it?");

    match color {
        Color::Red => println!("The color is Red"),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RGB(r, g, b) => println!("Red: {}, Blue: {}, Green: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, Magenta: {}, Yellow: {}, Key(Black): {}", c, m, y, k),
    }

}

fn references() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val)
    }

    match *reference {
        val => println!("Got a value via dereference: {:?}", val)
    }


    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

fn structs() {
    struct Foo {
        x: (u32, u32),
        y: u32
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
}

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn guards() {
    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn binding() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n@1..=12 => println!("I'm a child of age {:?}", n),
        n@13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm old person of age {:?}", n)
    }

    match some_number() {
        Some(n@42) => println!("The answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => ()
    }
}

fn main() {
    tuples();

    arrays_slices();

    enums();

    references();

    structs();

    guards();

    binding();

    let_else();

    while_let();
}
