type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of 'x' in bytes: {}", std::mem::size_of_val(&x));
    println!("size of 'y' in bytes: {}", std::mem::size_of_val(&y));
    println!("size of 'z' in bytes: {}", std::mem::size_of_val(&z));
    println!("size of 'i' in bytes: {}", std::mem::size_of_val(&i));
    println!("size of 'f' in bytes: {}", std::mem::size_of_val(&f));

    let elem = 5u8;

    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}", vec);

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}
