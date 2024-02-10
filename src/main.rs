mod demo {
    pub mod demo;
}

mod demo2;

//mod demo3;

mod demo4;

mod demo5;
use crate::demo5::log_mod;

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

    // 2.
    demo2::demo2::log_mod();

    // 3.
    //demo3::demo3::log_mod();

    // 4.
    demo4::log_mod();

    // 5.1
    demo5::log_mod();

    // 5.2
    log_mod();
}
