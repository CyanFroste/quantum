pub mod paths {
  const COMPONENTS: &str = "components";
  const SHARED_COMPONENTS: &str = "components/shared";
  pub const BUILD: &str = "build";
  pub const BUILD_COMPONENTS: &str = "build/components";

  // path builder for shared module
  pub fn shared(component: &str, module: &str) -> String {
    format!("{}/{}/_{}.scss", SHARED_COMPONENTS, component, module)
  }

  // iterative path builder for shared module !note: alphabetically ordered, hence unused
  // pub fn shared(component: &str) -> String {
  //   format!("{}/{}", SHARED_COMPONENTS, component)
  // }

  // path builder for source component
  pub fn components(component: &str, file: &str) -> String {
    match file {
      "html" => format!("{}/{}/index.html", COMPONENTS, component),
      "js" => format!("{}/{}/script.js", COMPONENTS, component),
      _ => format!("{}/{}/style.scss", COMPONENTS, component),
    }
  }
  
  // path builder for build components
  pub fn build(file: &str) -> String {
    match file {
      "js" => format!("{}/main.js", BUILD),
      "sass" => format!("{}/main.scss", BUILD),
      _ => format!("{}/{}.html", BUILD_COMPONENTS, file),
    }
  }
}
