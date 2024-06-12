// https://www.youtube.com/watch?v=DSZqIJhkNCM&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=6
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
// // enums can have different types as shown below
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// // you can do implementation blocks on enums like you can with structs
// impl Message {
//     fn some_function() {
//         println!("Print a line");
//     }
// }

// struct ApAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let four = IpAddrKind::V4(127, 0, 0, 1);
//     let six = IpAddrKind::V6(String::from("127.0.0.1"));

//     let localhost = IpAddrKind::V4(127, 0, 0, 1);

// }

// option enum - handles when a value does not exist since there is no 'null' value in rust
// enum Option<T> {
//     Some(T), // a generic value
//     None,    // stores no value
// }

// fn options() {
//     let some_number = Some(5);
//     let some_string = Some("a string");

//     let absent_number: std::prelude::v1::Option<i32> = None;
// }

// fn add_options() {
//     let x: i8 = 5;
//     let y: std::prelude::v1::Option<i8> = Some(5);
//     let z: std::prelude::v1::Option<i8> = None;

//     // this gives an error because we are trying to add a number Type and an Option type
//     let sum = x + y;
//     // to fix the scenario above we use the `unwrap_or` method - takes a default
//     let second_sum = x + y.unwrap_or(0);
// }

// Match Expressions
// mathces a value against a set of patterns
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
//     //...
// }

// enum UpdatedCoin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn updated_value_in_cents(coin: UpdatedCoin) -> u8 {
//     match coin {
//         UpdatedCoin::Penny => 1,
//         UpdatedCoin::Nickel => 5,
//         UpdatedCoin::Dime => 10,
//         UpdatedCoin::Quarter(state) => {
//             println!("The state is {:#?}", state);
//             25
//         }
//     }
// }

// you would call it like this
// updated_value_in_cents(Coin::Quarter(UsState::Alaska));

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is the answer {:#?}", six);
    println!("none is this: {:#?}", none);

    example();
}

// combine matched expression with a option enum
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // matching is exhaustive - we have to cover every scenario - you can use an underscore to handle all remaining cases
        // _ -> ()
    }
}

// we only care about one case - 3 - otherwise we do nothing
fn example() {
    let some_value = Some(3);

    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // shorthand way of writing the above function with if/let syntax
    // this is not exhasutive - we only specify the case that we care about
    if let Some(3) = some_value {
        println!("three");
    }
}
