use std::fmt::{Debug, Error};

struct Mtn {
  pub name: String,
  age: i32,
  options: Choices,
}

impl Debug for Mtn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("User")
      .field("name", &self.name)
      .field("age", &self.age)
      .finish()
  }
}

impl Mtn {
  fn getName(&self) -> String {
    let s = &self.name;
    s.to_string()
  }
}
enum Choices {
  A,
  B,
  C,
}

pub const PUZZLE: u32 = 5;
fn main() {
  println!("Hello, world!");
  let user = Mtn {
    name: String::from("Metin"),
    age: 35,
    options: Choices::A,
  };

  let ss: String = format!("{user:#?}");
  mtn_test();
  println!("MTn test lib {}", ss)
}

fn mtn_test() -> String {
  return "Test".to_string();
}
