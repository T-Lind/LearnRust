pub(crate) fn learn_vectors(){
    let mut vector_1 = Vec::new(); // Will be assigned to the first type pushed
    // vector_2 is now permanently of type i32
    let mut vector_2 = vec![1, 1, 2, 3, 5];

    // vector_1 is now permanently of f64
    vector_1.push(3.1);
    vector_1.push(3.9);
    vector_1.push(1.2);
    vector_1.pop();
    vector_1.push(-0.3);
    vector_1.push(-4.9);

    // Add this new inline vector to the end of the first
    vector_2.append(&mut vec![8, 13]);

    // Loop through the vector and print the data in it
    // Uses a normal for loop
    print!("Fib sequence: ");
    for a in 0..vector_2.len() {
        print!("{} ", vector_2[a]);
    }
    println!();

    // Enhanced for loop
    print!("Data in vector_1: ");
    for item in vector_1 {
        print!("{item} ")
    }


}