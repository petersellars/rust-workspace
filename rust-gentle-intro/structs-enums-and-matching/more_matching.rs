struct Point {
    x: f32,
    y: f32
}

// fn match_tuple(t: (i32,String)) {
//     let text = match t {
//         (0, s) => format!("zero {}", s),
//         (1, ref s) if s == "hello" => format!("hello one!"),
//         tt => format!("no match {:?}", tt),
//         // or say _ => format!("no match") if you're not interested in the value
//     };
//     println!("{}", text);
// }

fn main() {
  let t = (10, "hello".to_string());
  // let (n,s) = t; // t is moved, it is no more
  let (ref n, ref s) = t; // t is borrowed, it still lives

  println!("n is {}", n);
  println!("s is {}", s);

  let p = Point{x:1.0,y:2.0};
  let Point{x,y} = p; // p still lives, since x and y can still be copied

  println!("x is {}, y is {}", x, y);

  // Matchers

  // 2
  match (42, "answer") {
      (42, "answer") => println!("yes"),
      _ => println!("no")
  };

  // 3
  let ot = Some((2,"hello".to_string()));

  if let Some((_,ref s)) = ot {
      assert_eq!(s, "hello");
  }

  // 4
  if let Ok(n) = "42".parse::<i32>() { // _n = let variable be unused

  }

  // 5
  // let n: i32 = "42".parse()?; - use in a function
}