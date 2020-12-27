use super::utils::common::paths;
use super::utils::fs::{dir, file};
use super::utils::text::minify;

pub fn clean(shared_component: &str, components: &[&str]) -> bool {
  dir::remove(paths::BUILD);
  if dir::create(paths::BUILD_COMPONENTS) {
    return sass(shared_component, components) && markup(components) && js(components);
  }
  false
}

pub fn sass(shared_component: &str, components: &[&str]) -> bool {
  // copy modules from <shared>
  let mut sass = string!();

  // !note: alphabetically ordered, hence unused
  /* for file in dir::read(&paths::shared(shared_component)) {
    if file.contains("_module.scss") {
      continue;
    }
    sass.push_str(&file::read(&file));
  } */

  sass.push_str(&file::read(&paths::shared(shared_component, "colors")));
  sass.push_str(&file::read(&paths::shared(shared_component, "variables")));
  sass.push_str(&file::read(&paths::shared(shared_component, "typography")));
  sass.push_str(&file::read(&paths::shared(shared_component, "mixins")));
  sass.push_str(&file::read(&paths::shared(shared_component, "base")));

  if !file::append(&paths::build("sass"), &minify(&sass)) {
    return false;
  }
  // add scss from components
  for component in components {
    if !file::append(
      &paths::build("sass"),
      &minify(&file::read(&paths::components(component, "sass"))),
    ) {
      return false;
    }
  }
  true
}

pub fn markup(components: &[&str]) -> bool {
  // copy markup
  for component in components {
    if !file::write(
      &paths::build(component),
      &file::read(&paths::components(component, "html")),
    ) {
      return false;
    }
  }
  true
}

pub fn js(components: &[&str]) -> bool {
  for component in components {
    let js = minify(&file::read(&paths::components(component, "js")));
    if js.len() == 0 {
      continue;
    }
    if !file::append(&paths::build("js"), &format!("(function(){{ {} }})()", js)) {
      return false;
    }
  }
  true
}
