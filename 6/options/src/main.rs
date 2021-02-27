fn main() {
    let some_number = Some(5);
    println!("number: {:?}", some_number);
    let some_string = Some(String::from("hello"));
    println!("string: {:?}", some_string);

    let absent_number: Option<i32> = None;
    println!("absent: {:?}", absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // doesn't compile: cannot add `Option<i8>` to `i8`>
}
