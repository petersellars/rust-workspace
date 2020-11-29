// &str pronounced 'string slice'
fn dump(s: &str) {
    println!("str {}", s);
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    let text = "hello dolly"; // the string slice
    let s = text.to_string(); // it's now an allocated string

    dump(text);
    dump(&s);

    let mut s = String::new();
    // initially empty!
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for 'push_str'
    // remove the last char
    s.pop();

    assert_eq!(s, "Hello World");

    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10,20,30]");

    // slice notation with strings
    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("slices {:?} {:?}", text_s, string_s);

    // cannot index strings!
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        println!("{}", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }

    // string slicing may explode
    //let s = "¡";
    //println!("{}", &s[0..1]);  // bad, first byte of a multibyte character

    let text = "the red fox and the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("words {:?}", words);

    let mut words2 = Vec::new();
    // iterator into extend method
    words2.extend(text.split_whitespace());
    println!("words2 {:?}", words2);

    // iterator over the chars, only take non space characters
    // filter method takes a closure - lambda or anonymous function
    // Idiomatic filter over looping over....
    let stripped: String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect();
    println!("stripped {:?}", stripped);
}