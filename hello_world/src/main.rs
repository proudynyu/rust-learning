fn testing() {
    println!("This is a testing function");

    println!("{} days", 31);

    println!("{0} is friend of {1}. {1} likes to talk to {0}.", "Alice", "Bob");


    println!("{subject} {verb} {object}",
             object="The lazy dog",
             subject="the quick brown fox",
             verb="jumps over"
            );

}

fn numerical_values() {
    let choice_number: i32 = 69429;

    println!("Base 10:  {}", choice_number);
    println!("Base 2:  {:b}", choice_number);
    println!("Base 8:  {:o}", choice_number);
    println!("Base 16:  {:x}", choice_number);
}

fn shift_numbers() {
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0>width$}", number=1, width=5);
}

fn main() {
    println!("Hello, world!");
    println!("Testing, testing");

    testing();
    numerical_values();
    shift_numbers();
}


