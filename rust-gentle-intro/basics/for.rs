// for.rs
fn main() {
    for i in 0..5 {
        println!("Hello {}", i);
    }

    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }

    // Nice alternative without Ternary operator
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i)
    }
}