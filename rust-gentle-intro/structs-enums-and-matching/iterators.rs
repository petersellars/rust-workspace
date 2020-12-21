// Iterator Types
//
// Explicit
// for s in vec.iter() {...} // &String
// for s in vec.iter_mut() {...} // &mut String
//for s in vec.into_iter() {...} // String
//
// Implicit!
// for s in &vec {...} // &String
// for s in &mut vec {...} // &mut String
// for s in vec {...} // String

// for n in vec.iter().map(|x: &String| x.len()) {...} // n is usize

// for s in vec.iter().filter(|x: &&String| x.len() > 2) {...} // s is &String

// for s in vec.iter().filter(|&x| x == "one")