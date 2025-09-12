pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

pub struct User {
    name: String,
    level: AccessLevel,
}

impl User {

  pub fn new(name: String, level: AccessLevel) -> User {
    User {
        name: name,
        level: level,
    }
  }

  pub fn send_name(&self) -> Option<&str> {
    match self.level {
      AccessLevel::Guest => None,
      _ => Some(self.name.as_str())
    }
  }
  
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.level {
        AccessLevel::Guest => (false, "ERROR: User is guest"),
        _ => (true, &user.name)
    }
}