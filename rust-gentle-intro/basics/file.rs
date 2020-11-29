use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> io::Result<String> {
    // the ? here does almost exactly what the match in the function below does
    let mut file = File::open(&filename)?;
    // try! version
    // let mut file = try!(File::open(&filename));
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    // try! version
    // try!(file.read_to_string(&mut text));
    Ok(text)
}

// fn read_to_string(filename: &str) -> Result<String, io::Error> {
//     let mut file = match File::open(&filename) {
//         Ok(f) => f,
//         Err(e) => return Err(e),
//     };
//     let mut text = String::new();
//     match file.read_to_string(&mut text) {
//         Ok(_) => Ok(text),
//         Err(e) => Err(e),
//     }
// }

fn good_or_bad(good: bool) -> Result<i32,String> {
    if good {Ok(42)} else {Err("bad".to_string())}
}

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");

    let mut file = File::open(&first).expect("can't open the file");

    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");

    println!("file had {} bytes", text.len());

    println!("{:?}", good_or_bad(true));
    println!("{:?}", good_or_bad(false));

    match good_or_bad(true) {
        Ok(n) => println!("Cool I got {}", n),
        Err(e) => println!("Huh, I just got {}", e)
    }

    let file = env::args().nth(1).expect("please supply a filename");
    let text = read_to_string(&file).expect("bad file man!");
    println!("file had {} bytes", text.len());
}