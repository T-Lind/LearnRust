struct Number {
    odd: bool,
    prime: bool,
    value: i16,
}

pub(crate) fn learn_structures() {
    let n0 = Number {
        odd: true,
        prime: false,
        value: 16,
    };
    let mut n1 = Number {
        odd: true,
        prime: true,
        value: 15,
    };

    // Use the following format to avoid creating new variables when printing
    println!("Structure data: {}, {}, {}", n0.odd, n0.prime, n0.value);

    // n0 is not mutable, but n1 is!
    n1.prime = false;
    println!("Modified structure data: Is prime: {}", n1.prime) // Last line can skip the semicolon
}