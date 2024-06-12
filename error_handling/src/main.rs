// https://www.youtube.com/watch?v=wM6o70NAWUI&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=9
use std::fs::{self, File};
use std::io::{ErrorKind, Read};
fn main() {
    // immediately quits program
    // panic!("something went wrong, quit!");

    // Result enum handles success and err state
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let f = File::open("hello.txt");
    // redeclaring `f` is called shadowing
    // let f = match f {
    //     Ok(file) => file,
    //     // Err(error) => panic!("Problem openin ghte file: {:#?}", error),
    //     // more gracefully handle the error case
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file, {:#?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    // rather than using `match` in the code above, we can use CLOSURES (ch 13 in Rust book)
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // You can also use `unwrap`
    // let f = File::open("hello.txt").unwrap();
    // use `expect` to specify the error message
    let _f = File::open("hello.txt").expect("That file does not exist!");
}

// you can use `?` as a shortcut to propogate errors - returning them to the caller to handle
fn read_username_from_file() {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s);
    // the 3 lines of code above could also be written in one line - it has the error built in
    fs::read_to_string("hello.txt");
}
