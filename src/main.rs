mod demo {
    pub mod demo;
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg0 = &args[0];
    let arg1 = &args[1];

    // 0.
    println!("The first argument is : {}", arg0);
    println!("The second argument is : {}", arg1);

    // 1.
    demo::demo::log_mod();
}
