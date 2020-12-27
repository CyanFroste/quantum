pub mod dir {
  use std::fs;
  use std::path::{Path, PathBuf};

  pub fn remove(path: &str) -> bool {
    if let Ok(_) = fs::remove_dir_all(&Path::new(path)) {
      return true;
    }
    false
  }

  pub fn create(path: &str) -> bool {
    if let Ok(_) = fs::create_dir_all(&Path::new(path)) {
      return true;
    }
    false
  }

  pub fn read(path: &str) -> Vec<String> {
    let mut files = vec![];
    if let Ok(entries) = fs::read_dir(path) {
      for entry in entries {
        if let Ok(file) = entry {
          if let Ok(string) = file.path().into_os_string().into_string() {
            files.push(string);
          }
        }
      }
    }
    files
  }
}

pub mod file {
  use std::fs::{File, OpenOptions};
  use std::io::prelude::*;
  use std::path::Path;

  pub fn read(path: &str) -> String {
    let mut data = String::new();
    if let Ok(mut file) = File::open(&Path::new(path)) {
      if let Ok(_) = file.read_to_string(&mut data) {
        return data;
      }
    }
    data
  }

  pub fn write(path: &str, data: &str) -> bool {
    if let Ok(mut file) = File::create(&Path::new(path)) {
      if let Ok(_) = file.write_all(data.as_bytes()) {
        return true;
      }
    }
    false
  }

  pub fn append(path: &str, data: &str) -> bool {
    if let Ok(mut file) = OpenOptions::new()
      .create(true)
      .append(true)
      .open(&Path::new(path))
    {
      if let Ok(_) = file.write_all(data.as_bytes()) {
        return true;
      }
    }
    false
  }
}
