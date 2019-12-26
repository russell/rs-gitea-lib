/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AnnotatedTagObject : AnnotatedTagObject contains meta information of the tag object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotatedTagObject {
  #[serde(rename = "sha")]
  sha: Option<String>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>
}

impl AnnotatedTagObject {
  /// AnnotatedTagObject contains meta information of the tag object
  pub fn new() -> AnnotatedTagObject {
    AnnotatedTagObject {
      sha: None,
      _type: None,
      url: None
    }
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> AnnotatedTagObject {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> AnnotatedTagObject {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> AnnotatedTagObject {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

}



