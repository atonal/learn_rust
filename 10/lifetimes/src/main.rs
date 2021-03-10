use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // rule 1
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // rule 3 - return type gets same lifetime as &self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // static lifetime:
    let s: &'static str = "I live forever";

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("excerpt: {:?}", i);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // cannot return value referencing local variable `result`
    // let result = String::from("really long string");
    // result.as_str()
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
