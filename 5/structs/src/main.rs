#[derive(Debug)] // derive Debug trait to be used with {:?} or {:#?}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("user1"),
        sign_in_count: 23,
        active: true,
    };

    user1.email = String::from("user1@example.com");

    println!("user1: {:#?}", user1);

    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        ..user1
    };

    println!("user2: {:#?}", user2);

    let uname3 = String::from("user3");
    let email3 = String::from("user3@example.com");
    let user3 = build_user(uname3, email3);
    println!("user3: {:#?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}", black);
}

fn build_user(username: String, email: String) -> User {
    User {
        email,    // field init shorthand (variables have same names as fields)
        username, // field init shorthand
        sign_in_count: 23,
        active: true,
    }
}
