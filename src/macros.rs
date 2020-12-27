#[macro_use]
pub mod macros {
  macro_rules! log {
    ($($val:expr), *) => {
      $(
        println!("{}", $val);
      )*
    };
  }
  macro_rules! string {
    () => {
      String::new()
    };
    ($val:expr) => {
      String::from($val)
    };
  }
}