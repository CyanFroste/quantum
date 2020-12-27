#[macro_use]
mod macros;
mod build;
mod enums;
mod utils;

fn main() {
  if build::clean("default", &["sample"]) {
    log!("build successful!");
  } else {
    log!("build failed!");
  }  
}
