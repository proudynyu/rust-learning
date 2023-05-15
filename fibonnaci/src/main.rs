fn fibonnaci(n: u32) -> u32 {
    if n <= 2 {
        return 1
    }

    fibonnaci(n - 1) + fibonnaci(n - 2)
}

fn main() {
    for n in (1..10).rev() {
        println!("{}", fibonnaci(n));
    }
}
