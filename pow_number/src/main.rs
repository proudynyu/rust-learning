fn power(base: i32, pow: i32) -> i32 {
    if pow == 0 {
        return 1
    }
    return base * power(base, pow - 1)
}

fn main() {
   let pow = power(2, 2);
   let b = power(3, 2);

   println!("power of 2 is {}", pow);
   println!("power of 3 is {}", b);
}
