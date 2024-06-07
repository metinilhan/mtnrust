use std::fmt::Debug;

struct Mtn {
  pub name: String,
  age: i32,
  options: Vec<Choices>,
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
  fn get_name(&self) -> String {
    let name_formatted = format!("My user name is : {}", &self.name);
    name_formatted
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
    options: vec![Choices::A, Choices::B, Choices::C],
  };

  let ss: String = format!("{user:#?}");

  for opt in user.options.iter() {
    let res = match opt {
      Choices::A => String::from("A"),
      Choices::B => String::from("B"),
      Choices::C => String::from("C"),
    };
    println!("Test {}", res);
  }

  println!("user name :  {}", user.get_name());
  mtn_test();
  println!("MTn test lib {}", ss)
}

fn mtn_test() -> String {
  "Test".to_string()
}
