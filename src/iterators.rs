pub(crate) fn learn_iterators() {
    // Computes numbers 1-infinity lazily, so can be stored in RAM
    let natural_numbers = 1..;

    println!("Test condition 1: {}", natural_numbers.contains(&100));
    println!("Test condition 2: {}", (..=20).contains(&-30));
    println!("Test condition 3: {}", (3..6).contains(&6)); // Not normally inclusive (3, 4, 5)

    print!("String iterator to uppercase: ");
    for character in "rust".to_ascii_uppercase().chars() {
        print!("{character} ");
    }

    print!("\nAnother string iterator using filter & map: ");
    for character in "OowO INBoUND".chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase()) {
        print!("{character} ");
    }
    println!();
}