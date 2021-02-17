fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let condition = false;
    let num = if condition { 4 } else { 7 };
    println!("num: {}", num);
}
