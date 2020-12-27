pub enum Res {
  Ok(String),
  Err(String),
}

impl Res {
  pub fn ok(msg: &str) -> Self {
    Self::Ok(string!(msg))
  }
  pub fn err(msg: &str) -> Self {
    Self::Err(string!(msg))
  }
  pub fn msg(&self) -> String {
    match self {
      Self::Ok(msg) => string!(msg),
      Self::Err(msg) => string!(msg),
    }
  }
}
