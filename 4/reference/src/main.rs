fn main() {
    let s1 = String::from("hello");

    let len = calculate_len(&s1);

    println!("length of '{}' is {}", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 is {}", s2);

    let mut s3 = String::from("hello");
    {
        let r1 = &mut s3;
        println!("r1 {}", r1);
    }
    let r2 = &mut s3;
    println!("r2 {}", r2);

    let t1 = &s2; // no problem
    let t2 = &s2; // no problem
    println!("{} and {}", t1, t2);
    // t1 and t2 are no longer used after this point

    let t3 = &mut s2; // no problem
    println!("{}", t3);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
