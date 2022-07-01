#![allow(unreachable_code)]

fn main() {
    math();
    tup();
    tuple();
    array();

    let yy = {
        let x = 3;
        x + 1 // Expressions do not include ending semicolons
        // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    };
    println!("The value of yy is: {}", yy);
    if_else();
    loop_forever();
    loop_number_range();
    loop_break_with_return();
    loop_while();
    loop_iterator();
}

fn math() {
    println!("--- math() ---");
    let sum = 5 + 10;
    println!("sum {}", sum);

    let difference = 95.5 - 4.3;
    println!("difference {}", difference);

    let product = 4 * 30;
    println!("product {}", product);

    let quotient = 56.7 / 32.2;
    println!("quotient {}", quotient);
    let floored = 2 / 3;
    println!("floored {}", floored);

    let remainder = 43 % 5;
    println!("remainder {}", remainder);

}

fn tup() {
    println!("--- tup() ---");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    let (a, b, c) = (200, 5.4, 7);

    println!("The value of z,y,x is: {} {} {}", z,y,x);
    println!("The value of c,b,a is: {} {} {}", c,b,a);
    {
        let (x, y, z) = (a, b, c);
        println!("inner - The value of z,y,x is: {} {} {}", z,y,x);
    }
    println!("outer - The value of z,y,x is: {} {} {}", z,y,x);


}

fn tuple() {
    println!("--- tuple() ---");
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let tuple_first_element = tuple.0;
    println!("tuple_first_element {}", tuple_first_element);

    let tuple_seconds_element = tuple.1;
    println!("tuple_seconds_element {}", tuple_seconds_element);

    let tuple_third_element = tuple.2;
    println!("tuple_third_element {}", tuple_third_element);
}

fn array() {
    println!("--- array() ---");

    let array1 = [11, 12, 13, 14, 15];
    let array2: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array1 length {}", array1.len());

    let array1_element_0 = array1[0];
    println!("array1_element_0 {}", array1_element_0);

    for n in array2 {
        println!("{}", n);
    }

}
fn if_else() {
    println!("--- if_else() ---");
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}

fn loop_forever() {
    println!("--- loop_forever() ---");

    let mut count = 0u32;

    println!("Let's count until infinity! {}", 0u32);

    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            // Exit this loop
            break;
        }
    }
}

fn loop_number_range() {
    println!("--- loop_number_range() ---");
    for n in 0..23 {
        println!("{}", n);
    }
}

fn loop_nesting_label() {
    println!("--- loop_nesting_label() ---");
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
fn loop_break_with_return() {
    println!("--- loop_break_with_return() ---");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn loop_while() {
    println!("--- loop_while() ---");

    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        println!("{}", n);
        n += 1;
    }
}

fn loop_iterator() {
    println!("--- loop_iterator() ---");

    let names = vec!["Bob", "Frank", "Ferris"];

    /* iter()
    *  This borrows each element of the collection through each iteration.
    */ Thus leaving the collection untouched and available for reuse after the loop
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}
