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

fn main() {
    println!("Hello, world!");
}
