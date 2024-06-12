// https://www.youtube.com/watch?v=n3bPhdiJm9I&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=5
// the line below allows for the struct to be printed
#[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

struct Rectangle {
    width: u32,
    height: u32,
}

// implementation blocks are used to house methods of structs
// these are methods - tied to the data of the struct
impl Rectangle {
    // first arg in a method is always self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// you can use an associated function, which is not tied to the data in the struct
// these don't get passed `self` as the first arg
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let mut user1 = User {
    //     email: String::from("test@email.com"),
    //     username: String::from("billy"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let name = user1.username;
    // user1.username = String::from("bobby");

    // let user2 = build_user(String::from("Sally"), String::from("jonny@test.com"));

    // let user3 = User {
    //     email: String::from("tester@test.com"),
    //     username: String::from("jenny"),
    //     // below is like the spread syntax from JS - gives the remaining fields the same values as from user2
    //     ..user2
    // };

    // Tuple Structs - want it to have a name but don't want to identify each field
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    // calculate the area of a rectangle using structs
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    // {:?} prints the struct but adding `#` in the middle makes it more readable
    println!("The rect is {:#?}", rect);
    println!("The area of the rectangle is {} square feet", rect.area());

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("rect can hold react1:, {}", rect.can_hold(&rect1));
    println!("rect can hold react2:, {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(30);
    println!("The dimensions of rect3 are, {:#?}", rect3);
}

// fn build_user(username: String, email: String) -> User {
//     User {
//         username,
//         email,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// this func was replaced by the impl
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
