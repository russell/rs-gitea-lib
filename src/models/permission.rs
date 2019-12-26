/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Permission : Permission represents a set of permissions

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
  #[serde(rename = "admin")]
  admin: Option<bool>,
  #[serde(rename = "pull")]
  pull: Option<bool>,
  #[serde(rename = "push")]
  push: Option<bool>
}

impl Permission {
  /// Permission represents a set of permissions
  pub fn new() -> Permission {
    Permission {
      admin: None,
      pull: None,
      push: None
    }
  }

  pub fn set_admin(&mut self, admin: bool) {
    self.admin = Some(admin);
  }

  pub fn with_admin(mut self, admin: bool) -> Permission {
    self.admin = Some(admin);
    self
  }

  pub fn admin(&self) -> Option<&bool> {
    self.admin.as_ref()
  }

  pub fn reset_admin(&mut self) {
    self.admin = None;
  }

  pub fn set_pull(&mut self, pull: bool) {
    self.pull = Some(pull);
  }

  pub fn with_pull(mut self, pull: bool) -> Permission {
    self.pull = Some(pull);
    self
  }

  pub fn pull(&self) -> Option<&bool> {
    self.pull.as_ref()
  }

  pub fn reset_pull(&mut self) {
    self.pull = None;
  }

  pub fn set_push(&mut self, push: bool) {
    self.push = Some(push);
  }

  pub fn with_push(mut self, push: bool) -> Permission {
    self.push = Some(push);
    self
  }

  pub fn push(&self) -> Option<&bool> {
    self.push.as_ref()
  }

  pub fn reset_push(&mut self) {
    self.push = None;
  }

}


