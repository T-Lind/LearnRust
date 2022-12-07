extern crate core;

mod tuple;
mod string;
mod structures;
mod matches;
mod generics;
mod vectors;
mod enumerations;
mod iterators;

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

    println!();

    tuple::learn_tuple();

    println!();

    string::learn_string();

    println!();

    structures::learn_structures();

    println!();

    matches::learn_match(2);

    println!();

    generics::learn_generics();

    println!();

    vectors::learn_vectors();

    println!();

    enumerations::learn_enumerations();

    println!();

    iterators::learn_iterators();
}
