enum Result<T, E> {
    Ok(T),
    Err(E)
}

enum LiftHeight {
    ZERO = 0,
    LOW = 50,
    MID = 100,
    HIGH = 200,
}

pub(crate) fn learn_enumerations(){
    println!("Enumerations: ");
    // Notice the required casting
    println!("\nPrinting LiftHeight::MID: {}", LiftHeight::MID as i32);
    println!("Printing LiftHeight::HIGH: {}", LiftHeight::HIGH as i32);
    println!("Lift height of 50 = LOW: {}", LiftHeight::LOW as i32 == 50);
}