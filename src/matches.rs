pub(crate) fn learn_match(n: i32){
    print!("Printed value from match statement: ");
    if n > 0 {
        // Don't need breaks or anything strange
        match n {
            1 => println!("Value of 1"),
            2 => println!("Value of 2"),
            3 => println!("Value of 3"),
            _ => println!("Value of over 3 given")
        }
    }
    else {
        println!("Number outside of checking range!")
    }
}