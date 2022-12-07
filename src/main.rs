extern crate core;

mod tuple;
mod string;
mod structures;
mod matches;
mod generics;

fn dice_roll() -> i8 {
    return 3;
}



fn main() {
    let evaluate = {
        let a = 3;
        let b = 2;
        a + b * a
    };

    let x0 = 3;
    // 32 bits explicitly
    let x1: i32 = 3;
    println!("Numbers: {x0} {x1}");
    println!("let evaluation: {evaluate}");

    tuple::learn_tuple();

    string::learn_string();

    structures::learn_structures();

    matches::learn_match(2);

    generics::learn_generics();
}
