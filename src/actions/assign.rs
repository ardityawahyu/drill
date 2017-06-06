use std::collections::HashMap;

extern crate yaml_rust;
use self::yaml_rust::Yaml;

extern crate colored;
use self::colored::*;

extern crate serde_json;
use self::serde_json::Value;

use actions::Runnable;

#[derive(Clone)]
pub struct Assign {
  name: String,
  key: String,
  value: String,
}

impl Assign {
  pub fn is_that_you(item: &Yaml) -> bool{
    item["assign"].as_hash().is_some()
  }

  pub fn new(item: &Yaml, _with_item: Option<Yaml>) -> Assign {
    Assign {
      name: item["name"].as_str().unwrap().to_string(),
      key: item["assign"]["key"].as_str().unwrap().to_string(),
      value: item["assign"]["value"].as_str().unwrap().to_string()
    }
  }
}

impl Runnable for Assign {
  fn execute(&self, _base_url: &String, context: &mut HashMap<String, Yaml>, _responses: &mut HashMap<String, Value>) {
    println!("{:width$} {}={}", self.name.green(), self.key.cyan().bold(), self.value.magenta(), width=25);

    context.insert(self.key.to_owned(), yaml_rust::Yaml::String(self.value.to_owned()));
  }
}