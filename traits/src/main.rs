use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("Author")
    }
    fn summarize(&self) -> String {
        String::from("Not implemented!")
    }
}
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
    fn summarize_author(&self) -> String {
        format!("Author of the article: @{}", self.author)
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
        format!("{}, by {}", self.content, self.username)
    }
    fn summarize_author(&self) -> String {
        format!("Author of the tweet: {}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize())
}

// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {}
// pub fn notify2<T>(item1: &T, item2: &T) -> i32
// where
//     T: Display + Clone,
// {
//     return 2;
// }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

fn main() {
    let tw1 = Tweet {
        username: String::from("awril"),
        content: String::from("Hello there"),
        reply: false,
        retweet: false,
    };

    let nw1 = NewsArticle {
        author: String::from("awri.l"),
        headline: String::from("What a nice day!"),
        content: String::from("It is really great!"),
    };

    println!("{}", tw1.summarize());
    println!("{}", nw1.summarize());

    println!("\n{}", tw1.summarize_author());
    println!("{}", nw1.summarize_author());

    notify(&tw1);
}
