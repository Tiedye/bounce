use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn physics() -> String {
  "physics".into()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(physics(), "physics".to_string());
  }
}
