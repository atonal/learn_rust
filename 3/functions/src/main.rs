fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let y = {
        let z = 3;
        // no semicolon == expression
        z + 1
    };
    println!("The value of y is {}", y);

    let l = five();
    println!("The value of l is {}", l);

    let n = plus_one(5);
    println!("The value of n is {}", n);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}, y is {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
