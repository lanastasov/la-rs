mod demo {
    pub mod demo;
}

mod demo2;

//mod demo3;

mod demo4;

mod demo5;
use crate::demo5::log_mod;

mod demo6;

mod fake_bin {
    pub mod fake_bin;
}

mod greet_me {
    pub mod greet_me;
}

mod categorize_new_member {
    pub mod categorize_new_member;
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg0 = &args[0];
    // let arg1 = &args[1];

    // 0.
    println!("The first argument is : {}", arg0);
    // println!("The second argument is : {}", arg1);

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

    // 6
    demo6::log_mod();

    // 7.
    println!("{:?}", fake_bin::fake_bin::fake_bin("45385593107843568"));

    // 8.
    println!("{:?}", greet_me::greet_me::greet("riley"));
    println!("{:?}", greet_me::greet_me::greet("JACK"));

    // 9.
    println!(
        "{:?}",
        categorize_new_member::categorize_new_member::open_or_senior(vec![
            (45, 12),
            (55, 21),
            (19, -2),
            (104, 20),
        ])
    );
}
