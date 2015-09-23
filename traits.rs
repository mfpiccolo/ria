fn main() {
  let john = User { name: "John".to_string(), password: "1234".to_string() };
  let phil = Admin { name: "John".to_string(), password: "12345".to_string() };
  phil.authenticate();
}

trait Auth {
  fn authenticate(&self);
  fn password(&self) -> &String;
}

struct User {
  name: String,
  password: String,
}

struct Admin {
  name: String,
  password: String,
}

impl Auth for User {
  fn authenticate(&self) {
    authenticate(self);
  }
  fn password(&self) -> &String {
    &self.password
  }
}

impl Auth for Admin {
  fn authenticate(&self) {
    authenticate(self);
  }
  fn password(&self) -> &String {
    &self.password
  }
}

// a free-standing function that converts a (borrowed) point to a string
fn authenticate<T: Auth>(person: &T) {
  if (person.password().len() > 4) {
    println!("{}", "You have been authenticated!");
  } else {
    println!("{}", "Sorry, try a longer password.");
  }
}

