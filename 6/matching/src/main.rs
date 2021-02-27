#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    Colorado,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let mut val = value_in_cents(Coin::Penny);
    println!("value: {}", val);
    val = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("value: {}", val);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
}
