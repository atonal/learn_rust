fn main() {
    // loop {
    //     println!("Hello, world!");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("ther result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("number: {}", number);

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value at index {} is {}", index, a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is {}", element);
    }

    for number in (1..4).rev() {
        println!("the number is {}", number);
    }
}
