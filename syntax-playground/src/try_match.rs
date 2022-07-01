pub fn match_number() {
    println!("--- match_number() ---");
    for number in 0..30 {
        match number {
            // Match a single value
            1 => println!("{} is One!", number),
            // Match several values
            2 | 3 | 5 | 7 | 11 | 13 => println!("{} is a prime", number),
            // Match an inclusive range
            10..=19 => println!("{} is a teen", number),
            // Handle the rest of cases
            _ => println!("{} is not special", number), // if missing this line will error
        }
    }
}

pub fn match_bool() {
    println!("--- match_bool() ---");
    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1, // if missing this line will error
    };

    println!("{} -> {}", boolean, binary);
}
