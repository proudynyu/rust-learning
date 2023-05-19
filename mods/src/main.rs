mod split;

use split::split;

mod my_mod {
    fn private_function() {
        print!("called my_mod::private_function()")
    }

    pub fn function() {
        print!("called my_mod::function()")
    }

    pub fn indirect_access() {
        println!("called my_mod::indirect_access()");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called my_mod::nested::private_function()");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called my_mod::nested::publuc_function_in_my_mod()");
            public_function_in_my_mod();
        }

        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested()");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod()");
        }
    }
}

mod new_mod {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T
    }

    impl <T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents,
            }
        }
    }
}

fn main() {
    let open_box = new_mod::OpenBox { contents: "public information" };
    println!("The open box contains: {}", open_box.contents);

    // let closed_box = new_mod::ClosedBox { contents: "private information" };

    let closed_box = new_mod::ClosedBox::new("private information");

    split();
}
