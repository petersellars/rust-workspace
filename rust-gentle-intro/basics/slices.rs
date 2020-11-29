// read as: slice of i32
// Rust names '&' as borrowed
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [10, 20, 30, 40];
    // look at that &
    let res = sum(&arr);
    println!("sum {}", res);

    let ints = [1, 2, 3, 4, 5];
    // borrowing
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..]; // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);

    // Optional Values

    let slice = &ints;
    let first = slice.get(0);
    //let last = slice.get(5);
    let maybe_last = slice.get(5);
    // check is_some() before unwrapping
    //let last = if maybe_last.is_some() {*maybe_last.unwrap()} else {-1};
    // shortcut
    let last = *maybe_last.unwrap_or(&-1);

    println!("first {:?}", first);
    println!("maybe_last {:?}", maybe_last);
    println!("last {:?}", last);

    println!("first {} {}", first.is_some(), first.is_none());
    println!("maybe_last {} {}", maybe_last.is_some(), maybe_last.is_none());
    println!("first value {}", first.unwrap());
}