// https://www.youtube.com/watch?v=6rcTSxPJ6Bw&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=10
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is: {}", largest);
    // what if we are asked to find the largest number in 2 different lists of numbers?
    // we would have to duplicate this code, or abstract the logic into a function
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    // same as above with a function
    let number_list = vec![34, 50, 25, 100, 65];

    let result = get_largest(&number_list);
    println!("result of function is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = get_largest(&number_list);
    println!("result of function is {}", result);

    // use generic function to handle numbers and chars
    let list = vec!["a", "z", "d", "j"];
    let result = get_largest_gen(list);
    println!("result of function is {}", result);

    use_point_three()
}

// create a function to handle the same logic
fn get_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// but what if we want to compare some characters, not numbers?
// Then we make a function that can take in a generic Type (T)
fn get_largest_gen<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

// Geneics in structs
struct Point {
    x: i32,
    y: i32,
}

fn get_point() {
    let p1 = Point { x: 5, y: 9 };
    // this doesn't work because they are Floating Point numbers instead of Integers
    let p2 = Point { x: 5.0, y: 9.5 };
}
// to fix, add Generic Type
struct PointTwo<T> {
    x: T,
    y: T,
}
// impl block to return the x field - the U could also be T or any other char, this just shows that it is scoped to its own function and not the T from the struct above
impl<U> PointTwo<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

fn get_new_point() {
    let p1 = PointTwo { x: 5, y: 10 };
    let p2 = PointTwo { x: 5.0, y: 9.5 };
    // this fails because it is expecting the same type for both
    let p3 = PointTwo { x: 5, y: 10.2 };
}

#[derive(Debug)]
struct PointThree<T, U> {
    x: T,
    y: U,
}
// impl block to take x from self and y from other
impl<T, U> PointThree<T, U> {
    fn mix_up<V, W>(self, other: PointThree<V, W>) -> PointThree<T, W> {
        PointThree {
            x: self.x,
            y: other.y,
        }
    }
}

fn get_point_three() {
    let p1 = PointThree { x: 5, y: 10 };
    let p2 = PointThree { x: 5.0, y: 9.5 };
    // this works now since we added a different generic type in the struct
    let p3 = PointThree { x: 5, y: 10.2 };
}

fn use_point_three() {
    let p1 = PointThree { x: 5, y: 5.7 };
    let p2 = PointThree {
        x: "Hello",
        y: "There",
    };

    let p3 = p1.mix_up(p2);
    println!("The answer for p3 is: {:#?}", p3);
}
// Generics for Enums
// we have used these in Options and Errors
