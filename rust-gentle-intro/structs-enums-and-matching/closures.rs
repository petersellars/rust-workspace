fn main() {
    let f = |x| x * x;
    let res = f(10);
    println!("res {}", res);

    let m = 2.0;
    let c = 1.0;
    let lin = |x| m*x + c; // Closure - Can't do this in function
    println!("res {} {}", lin(1.0), lin(2.0));

    // Closure lifecycle
    let mut s = "world";
    {
        let mut changer = || s = "world";
        changer();
    }
    assert_eq!(s, "world");

    let name = "dolly".to_string();
    let age = 42;
    let cname = name.to_string(); // pass cloned copy of name

    let c = move || {
        println!("name {} age {}", cname, age);
    };

    c();

    println!("name {}", name);
    // Good for when creating threads

    // Iterators
    // let sin: Vec<f64> = range(0.0,1.0,0.1).map(|x| x.sin()).collect();
    // let sum: f64 = range(0.0,1.0,0.1).map(|x| x.sin()).sum();

    // filter
    let tuples = [(10,"ten"),(20,"twenty"),(30,"thirty"),(40,"forty")];
    let iter = tuples.iter().filter(|t| t.0 > 20).map(|t| t.1);
    for name in iter{
        println!("{}", name);
    }
}

// let f = |x| x * x
// fixes f to i32
// so..equal to...
// fn f (x: i32) -> i32 {
//    x * x
// }

// Closures is a struct and borrows values - has a lifetime