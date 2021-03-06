fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    scores.insert(String::from("Yellow"), 40); // overwrites
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(200);
    scores.entry(String::from("Green")).or_insert(400); // only this is added
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("field_name: {}", field_name); // fails: borrow of moved value: `field_name`

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(val) => println!("score: {}", val),
        None => (),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut words = HashMap::new();
    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", words);
}
