pub struct Password {
  pub password: String
}

impl Password {
  pub fn new(password: &str) -> Password {
    Password { password: password.to_string() }
  }
}