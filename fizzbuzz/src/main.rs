fn fizz_buzz(interactions: u32) {
    let mut vec: Vec<String> = Vec::new();

    for x in 1..=interactions {
        let mut str = String::from("");

        if x % 3 == 0 {
           str.push_str("fizz");
        }

        if x % 5 == 0 {
           str.push_str("buzz");
        }

        vec.push(if str.len() > 0 { str } else { x.to_string() });
    }

    println!("{:?}", vec);
}

fn main() {
    fizz_buzz(10);
    fizz_buzz(20);
    fizz_buzz(15);
    fizz_buzz(100);
}
