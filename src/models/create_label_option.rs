/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateLabelOption : CreateLabelOption options for creating a label

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLabelOption {
  #[serde(rename = "color")]
  color: String,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "name")]
  name: String
}

impl CreateLabelOption {
  /// CreateLabelOption options for creating a label
  pub fn new(color: String, name: String) -> CreateLabelOption {
    CreateLabelOption {
      color: color,
      description: None,
      name: name
    }
  }

  pub fn set_color(&mut self, color: String) {
    self.color = color;
  }

  pub fn with_color(mut self, color: String) -> CreateLabelOption {
    self.color = color;
    self
  }

  pub fn color(&self) -> &String {
    &self.color
  }


  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> CreateLabelOption {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> CreateLabelOption {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



