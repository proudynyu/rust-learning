pub mod nested;
mod inaccesible;

pub fn function() {
    println!("called split::function()");
}

fn private_function() {
    println!("called split::private_function()");
}

pub fn indirect_access() {
    private_function();
    println!("called split::indirect_access()");
}
