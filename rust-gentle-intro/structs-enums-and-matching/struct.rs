use std::fmt;

// Enables debug dump of Person
// #[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

// trait for debug - replace $[derive(Debug)]
impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

impl Person {
    // no self can associate functions with structs
    // like this 'new' constructor
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    // &self use the values, but not change them
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // &self use the values, but not change them
    fn copy(&self) -> Self {
        Self::new(&self.first_name,&self.last_name)
    }

    // &mut self can modify the value
    fn set_first_name(&mut self, name:&str) {
        self.first_name = name.to_string();
    }
    
    // self moves the value
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

fn main() {
    let p = Person::new("John","Smith");
    println!("person {} {}", p.first_name, p.last_name);
    println!("fullname {}", p.full_name());
    
    //let mut p2 = Person::copy(&p);
    let mut p2 = p.copy();
    println!("copy {}", p2.full_name());
    
    //Person::set_first_name(&mut p2, "Steve");
    p2.set_first_name("Steve");
    println!("Mutated firstname {}", p2.full_name());
    println!("{:?}", p2);
    
    //let (first, last) = Person::to_tuple(p2);
    let (first, last) = p2.to_tuple();
    println!("Tuple {} {}", first, last);

    println!("{:?}", p);
    // can not dump p2 as it has moved
    //println!("{:?}", p2);
}