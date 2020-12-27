pub fn minify(text: &str) -> String {
  let mut temp = String::new();
  for line in text.lines() {
    if line.trim().starts_with("//") || line.trim().len() == 0 {
      continue;
    }
    temp.push_str(line)
    // temp.push_str(&line[..line.find("//").unwrap_or(line.len())].trim());
  }
  temp
}

fn is_between(pattern: &str, symbol: char, string: &str) -> bool {
  let occurences: Vec<_> = string.match_indices(symbol).collect();
  let count = occurences.len();
  // check if pattern is b/w the first and last occurence for now
  if count >= 2 {
    if string[occurences[0].0..occurences[count - 1].0].contains(pattern) {
      return true;
    }
  }
  false
}
