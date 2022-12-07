struct Data {
    value: i32
}


fn maximum<T: PartialOrd>(a: T, b: T) -> T{
    if a > b {
        return a;
    }
    return b
}

pub(crate) fn learn_generics() {

    println!("Maximum value through a generic: {}", maximum(3.2, -4.1));
}
