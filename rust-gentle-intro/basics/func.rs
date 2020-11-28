fn sqr(x: f64) -> f64 {
    // return x * x - return can be explicit with last expression 
    // note: no ; needed with this approach
    x * x
}

fn abs(x: f64) -> f64 {
   if x > 0.0 {x} else {-x}
}

// ensure the number falls in a given range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {x1} else {
        if x > x2 {x2} else {x}
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {1} else {n * factorial(n-1)}
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);
    
    let abs_res = abs(-2.0);
    println!("abs is {}", abs_res);

    let clamp_res = clamp(7.0, 5.0, 10.0);
    println!("clamp is {}", clamp_res);

    let factorial_res = factorial(3);
    println!("factorial is {}", factorial_res);

    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);

    let mut res3 = 0.0;
    modifies(&mut res3);
    println!("res is {}", res3);

    let bigint: i64 = 42;
    println!("typed is {}", bigint);
}