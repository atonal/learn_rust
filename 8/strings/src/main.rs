fn main() {
    let mut a = String::new();

    let data = "initial contents";
    let b = data.to_string();

    let c = "foo bar".to_string();

    let hello = String::from("你好");

    let mut d = String::from("foo");
    d.push_str("bar");
    println!("{}", d);

    let mut e = String::from("lo");
    e.push('l');
    println!("{}", e);

    let f = String::from("hello ");
    let g = String::from("world");
    let h = f + &g; // note f has been moved here and can no longer be used
    println!("{}", h);

    let i = String::from("tic");
    let j = String::from("tac");
    let k = String::from("toe");
    let l = i + "-" + &j + "-" + &k;
    println!("{}", l);

    let m = String::from("tic");
    let n = String::from("tac");
    let o = String::from("toe");
    let p = format!("{}-{}-{}", m, n, o);
    println!("{}", p);

    let hello2 = "Здравствуйте";
    let q = &hello2[0..4];
    println!("{}", q);

    // .chars() gives Unicode scalar values
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
