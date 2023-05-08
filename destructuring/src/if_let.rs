enum Foo {
    Bar, 
    Baz, 
    Qux(u32)
}

pub fn if_let_enum() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred {}", value);
    }
}

pub fn fix_this() {
    // FIXED
    // This enum purposely neither implements nor derives PartialEq.
    // That is why comparing Foo::Bar == a fails below.
    enum Foo {Bar}

    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar");
    }
}

pub fn if_let_match() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32>  = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letter = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letter {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't lie letter. Let's go with an emoticon :)!");
    }
}

