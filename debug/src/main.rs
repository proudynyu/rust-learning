use std::fmt;

struct SideStructure(i32);

impl fmt::Display for SideStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i32, i32);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "real: {}, imag: {}i", self.real, self.imag)
   }
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}", 
             small=small_range,
             big=big_range
            );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(3)));

    let name = "Peter";
    let age = 27;
    let peter =  Person { name, age };

    println!("{:#?}", peter);

    let complex = Complex { real: 1.0, imag: 3.0 };
    println!("Compare Complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
