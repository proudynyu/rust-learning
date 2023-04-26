#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: point_left, bottom_right: point_right } = rect;
    let x_distance = point_right.x - point_left.x;
    let y_distance = point_right.y - point_left.y;

    x_distance.abs() * y_distance.abs()
}

fn square(point: Point, scalar: f32) -> Rectangle {
    Rectangle {
        top_left: Point { ..point },
        bottom_right: Point { x: point.x / scalar, y: point.y / scalar }
    }
}

enum WebEvent {
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnLoad => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => { println!("clicked at x={}, y={}", x, y); },
    }
}

enum VeryVerboseEnumOfThingsTodoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsTodoWithNumbers;

impl VeryVerboseEnumOfThingsTodoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({},{})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rec = Rectangle {
        top_left: Point { x: 0.0, y: 10.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };

    println!("Area for the Rectangle: {:?}", rect_area(rec));

    let rectangle_square = square(point, 2.0);
    println!(
        "Rectangle created with square: {:?}", 
        rectangle_square
    );

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
