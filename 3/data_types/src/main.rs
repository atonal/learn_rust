fn main() {
    let dec = 34_5;
    let hex = 0x54a;
    let oct = 0o077;
    let bin = 0b1100_0011;
    let byte = b'a';
    let byte_lit = b"aA";
    println!("dec: {}", dec);
    println!("hex: {}", hex);
    println!("oct: {}", oct);
    println!("bin: {}", bin);
    println!("byte: {}", byte);
    println!("byte_lit: {:#?}", byte_lit);

    let f = 2.4;
    let g: f32 = 0.6;
    println!("f: {}", f);
    println!("g: {}", g);

    let t = true;
    let w: bool = false;
    println!("t: {}", t);
    println!("w: {}", w);

    let tup: (i32, f64, bool) = (2, 4.5, false);
    // println!("tup: {}", tup);
    println!("tup: {:?}", tup);
    println!("tup: {:#?}", tup);

    let (x, y, z) = tup;
    println!("x, y, z: {}, {}, {}", x, y, z);

    let u = tup.0;
    let v = tup.2;
    println!("u, v: {}, {}", u, v);

    let mut a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("b: {:?}", b);

    // ['a', 'a', 'a', 'a', 'a', 'a']
    let c = ['a'; 6];
    println!("c: {:?}", c);

    let d: [u8; 2] = [0; 2];
    println!("d: {:?}", d);

    let first = a[0];
    let second = a[1];
    a = [7, 8, 9, 10, 11];
    println!("first: {:?}", first); // first: 1
    println!("second: {:?}", second); // second: 2

    // let seventh = a[7];
    // println!("seventh: {:?}", seventh);
}
