#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq,PartialOrd)]
enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 50,
    Faster
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right"
        }
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}

fn main() {
    let start = Direction::Left;
    println!("start {:?}", start);
    println!("{}", start.as_str());

    let mut d = start;
    for _ in 0..8 {
        println!("d {:?}", d);
        d = d.next();
    }

    let my_dir = Direction::Left;
    assert_eq!(my_dir, Direction::Left);

    let s = Speed::Slow;
    let speed = s as u32;
    println!("speed {}", speed);
    let f = Speed::Faster;
    let faster = f as u32;
    println!("faster {}", faster);
}