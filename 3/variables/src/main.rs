fn main() {
    const MAX_POINTS: u32 = 100_000;
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    // shadowing
    let y = 5;
    let y = y + 1;
    println!("The value of y is {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);
}
