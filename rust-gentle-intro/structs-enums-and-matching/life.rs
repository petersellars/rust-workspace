#[derive(Debug)]
struct A {
    s: &'static str
}

#[derive(Debug)]
struct B <'b> {
    s: &'b str
}

fn how(i: u32) -> &'static str {
    match i {
        0 => "none",
        1 => "one",
        _ => "many"
    }
}

// fn makes_b() -> B<'static> {
//     let string = "I'm a little string".to_string();
//     B { s: &string }
// }

fn main() {
    let a = A { s: "hello dammit" };
    
    println!("{:?}", a);
    println!("{:?}", how(1));

    let s = "I'm a little string".to_string();
    let b = B { s: &s };

    println!("{:?}", b);
}