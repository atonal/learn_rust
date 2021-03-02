fn main() {
    let a: Vec<i32> = Vec::new();
    let b = vec![1, 2, 3];

    let mut c = Vec::new();
    c.push(5);
    c.push(6);
    c.push(7);
    c.push(8);

    let d = vec![1, 2, 3, 4, 5];

    let third: &i32 = &d[2];
    println!("the third element is {}", third);

    match d.get(2) {
        Some(third_val) => println!("the third element is {}", third_val),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &d[100];
    let does_not_exist = d.get(100);

    let mut e = vec![1, 2, 3, 4, 5];
    let first = &e[0];
    // e.push(6); // mutable borrow when there is immutable borrow on the line above
    println!("the first element is {}", first);

    let f = vec![100, 34, 45];
    for i in &f {
        println!("{}", i);
    }

    let mut g = vec![48, 39, 59];
    for i in &mut g {
        *i += 100;
    }
    println!("{:?}", g);

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(34),
        SpreadSheetCell::Float(0.23),
        SpreadSheetCell::Text(String::from("foo bar")),
    ];

    println!("row: {:?}", row);
}
