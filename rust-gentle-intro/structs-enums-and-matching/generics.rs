// struct Foo {
//     name: String
// }

fn dump<T> (value: &T)
where T: std::fmt::Debug {
    println!("value is {:?}", value);
}

fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul + Copy {
    x * x
}

fn main() {
    let n = 42;
    dump(&n);

    // let foo = Foo{name: "hello".to_string()};
    // dump(&foo);

    let res = sqr(10.0);
    println!("res {}", res);
}