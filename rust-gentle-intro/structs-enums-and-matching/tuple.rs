fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x* y)
}

fn main() {
    let t = add_mul(2.0, 10.0);

    // can debug print
    println!("t {:?}", t);

    // can 'index' the values
    println!("add {} mul {}", t.0, t.1);

    // can _extract_ values
    let (add, mul) = t;
    println!("add {} mul {}", add, mul);

    let mtuple = ("hello", 5, 'c');
    assert_eq!(mtuple.0, "hello");
    assert_eq!(mtuple.1, 5);
    assert_eq!(mtuple.2, 'c');

    for t2 in ["zero", "one", "two"].iter().enumerate() {
        print!(" {} {};", t2.0, t2.1);
    }
    print!("\n");

    // zip combines two iterators into a single iterator of tuples
    // containing values from both:
    let names = ["ten","hundred","thousand"];
    let nums = [10,100,1000];
    for p in names.iter().zip(nums.iter()) {
        print!(" {} {};", p.0, p.1)
    }
    print!("\n");
}