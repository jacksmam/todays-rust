//コード8-12

// fn main() {
//   struct Color(f32, f32, f32);
//   let yellow = Color(1.0, 1.0, 0.0);
//   println!("R:{:.2} G:{:.2} B:{:.2}", yellow.0, yellow.1, yellow.2);
// }


//コード8-14
struct Person {
  id: i32,
  name: String,
  age: i32,
  addr: String,
}

impl Person {
  fn print(&self) {
    println!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
  }
}

fn main() {
  let pa = Person {
    id: 1,
    name: String::from("masuda"),
    age: 50,
    addr: String::from("Tokyo"),
  };

  pa.print();
}
