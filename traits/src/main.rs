// https://www.youtube.com/watch?v=T0Xfltu4h3A&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=11
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait defines that the summarize fn returns a string - leaves implementation details for the impls
pub trait Summary {
    fn summarize(&self) -> String {
        // this body is default but is over ridden by the impls
        String::from("(Read more...)")
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Bitcoin hits $1,000,000!"),
        location: String::from("Provo, UT"),
        author: String::from("John Doe"),
        content: String::from("Finally it hits the millions"),
    };

    let tweet = Tweet {
        username: String::from("@billybob"),
        content: String::from("Solana takes over the world"),
        reply: false,
        retweet: false,
    };

    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());

    notify(&article);
}

// you can use traits in parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// the above function is shorthad for what is below which is known as `trait bound`
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// // multiple params
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//     // ...
// }
// // or with trait bound format
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//     // first and second item have to be the same type
// }

// // with more that just one trait
// pub fn notify(item: &(impl Summary + Display)) {
//     // now there is a Display trait as well
// }
// // with tait bound format
// pub fn notify<T: Summary + Display>(item: &T) {}

// //sometimes it can become not very readable
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// // so we can use the `where` clause
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// you can also have a Trait as a return type
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// using trait bounds to conditionally implement methods
use std::fmt::Display;

struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
