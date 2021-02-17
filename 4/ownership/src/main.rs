fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error: value moved
    println!("{}", s2);

    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);

    let s4 = String::from("foo"); // s4 comes into scope

    takes_ownership(s4); // s4 moves into fn and is no longer valid here

    // println!("{}", s4); // error: value moved

    let num = 43; // num comes into scope

    makes_copy(num); // num is copied into fn
    println!("{}", num);

    let s5 = gives_ownership(); // moves it reutrn value to s5
    println!("{}", s5);

    let s6 = String::from("world!");

    println!("{}", s6);
    let s7 = takes_and_gives_back(s6); // s6 is moved to fn, which then moves it to s7

    // println!("{}", s6); // error: value moved
    println!("{}", s7);
} // s5 and s7 goes out of scope and are dropped. s6 was already dropped.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("some string: {}", some_string);
} // some_string goes out of scope and `drop` is called and the memory is free'

fn makes_copy(some_number: i32) {
    println!("some number: {}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("world");
    some_string // some_string is returned and moves out of the function
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string // some_string is returned and moves out of the function
}
