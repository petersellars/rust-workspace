use std::env;

fn main() {
    for arg in env::args() {
        println!("{}", arg);
    }

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        println!("{:?}", args);
    }

    // Idiomatic Rust -- note: this can panic!!
    let first = env::args().nth(1).expect("please supply an arguement");
    let n: i32 = first.parse().expect("not an integer!"); 
    // do your magic...
    println!("{}", n);
}