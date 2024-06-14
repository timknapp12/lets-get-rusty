// https://www.youtube.com/watch?v=juIINGuZyBc&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=12
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
// the borrow checker does not know the lifetime of both x and y so throws an error - we need to use a `generic lifetime annotation`
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// this means that the lifetime of the returned item will be equal to the smallest lifetime of either x or y (which ever is shortest)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// you can use lifetimes with structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn not_main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// This struct has the single field part that holds a string slice, which is a reference. As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

// The main function here creates an instance of the ImportantExcerpt struct that holds a reference to the first sentence of the String owned by the variable novel. The data in novel exists before the ImportantExcerpt instance is created. In addition, novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.

// Lifetime Elision rules makes it so Rust handles some of this for us so we don't have to in every case
// Rule 1 - Each parameter that is a reference gets its own lifetime parameter
// Rule 2 - If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// Rule 3 - If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime is assigned to all output lifetime parameters

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Rule 3 applies here
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}


// - Static Lifetime - live as long as the program - like string literals - because they live in the programs binary
let s: &'static str = "I have a static lifetime.";

// Generics, Traits, and Lifetime all put together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
