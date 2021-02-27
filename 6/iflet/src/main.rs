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

fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three again");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("state of quarter: {:?}", state),
        _ => count += 1,
    }

    let coin2 = Coin::Quarter(UsState::Colorado);
    if let Coin::Quarter(state) = coin2 {
        println!("state of quarter: {:?}", state);
    } else {
        count += 1;
    }

    println!("count: {}", count);
}
