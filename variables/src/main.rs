// https://www.youtube.com/watch?v=2V0JaMVjzws&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=3
fn main() {
    let mut x = 5;
    println!("The variable is: {}", x);
    x = 6;
    println!("The variable is: {}", x);

    // CONSTs cannot ever be mutable, and can't be returned from a function at run time, and must be assigned a type
    const COUNT: u32 = 100_000;
    println!("{}", COUNT);

    second();

    let sum = my_function(11, 12);
    println!("the added value in the sum is: {}", sum);

    standard_loop();

    while_loop();

    for_loop();

    ownership();
}

// rust has 4 scalar data types
// 1 Integers
// 2 Floating-point numbers => numbers with decimals
// 3 Booleans
// 4 Characters

// rust has 2 compound types
// 1 Tuple - A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
// you can destructure or use dot notation to get values from tuples
fn second() {
    let tup: (&str, i32) = ("Let's get rusty!", 100_000);
    // destrucute to get the values out
    let (first, second) = tup;
    println!("{}", first);
    println!("{}", second);
}
// functions are lower case and use underscores
// functions can return statements like printing a string or expressions
// arrow shows the return type
fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let sum = x + y;
    return sum;
}

// if else statements must be conditioned upon Booleans

// 3 types of loops
// 1 standard
// 2 while loop
// 3 for loop - looping through a collection - like an array
fn standard_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("result is {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println! {"number is {}", number};
        number -= 1;
    }
    println!("Lift off cuz number is {}", number);
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The element is {}", element);
    }

    for number in 1..8 {
        println!("The number is {}", number);
    }
}

/*
    this is a blcok comment
    with multiple lines
*/

// Chapter 4: Ownership
// https://www.youtube.com/watch?v=VFIOSWy93H0&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=4
fn ownership() {
    let x: i32 = 5;
    let _y: i32 = x; // Copy - integers can be copied

    let s1: String = String::from("hello");
    // let s2: String = s1; // Move (not copy) because strings cannot be copied
    let s3: String = s1.clone(); // Clone instead of move
    println!("{}, world", s1);
    println!("{}, world", s3);
}
