use trait_objects::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // redacted
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 100,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 75,
                height: 100,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // let screen2 = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };

    // screen2.run();
}
