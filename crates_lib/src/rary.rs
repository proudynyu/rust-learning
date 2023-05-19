pub fn public_function() {
    println!("called public_function()");
}

fn private_function() {
    println!("called private_function()");
}

pub fn indirect_access() {
    println!("called indirect_access with: ");
    private_function();
}
