fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear();

    println!("the first word is {}", word);

    let lit = "string literal";

    let buli = first_word(lit);

    println!("the first word is {}", buli);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
