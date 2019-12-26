/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PrBranchInfo : PRBranchInfo information about a branch

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrBranchInfo {
  #[serde(rename = "label")]
  label: Option<String>,
  #[serde(rename = "ref")]
  _ref: Option<String>,
  #[serde(rename = "repo")]
  repo: Option<::models::Repository>,
  #[serde(rename = "repo_id")]
  repo_id: Option<i64>,
  #[serde(rename = "sha")]
  sha: Option<String>
}

impl PrBranchInfo {
  /// PRBranchInfo information about a branch
  pub fn new() -> PrBranchInfo {
    PrBranchInfo {
      label: None,
      _ref: None,
      repo: None,
      repo_id: None,
      sha: None
    }
  }

  pub fn set_label(&mut self, label: String) {
    self.label = Some(label);
  }

  pub fn with_label(mut self, label: String) -> PrBranchInfo {
    self.label = Some(label);
    self
  }

  pub fn label(&self) -> Option<&String> {
    self.label.as_ref()
  }

  pub fn reset_label(&mut self) {
    self.label = None;
  }

  pub fn set__ref(&mut self, _ref: String) {
    self._ref = Some(_ref);
  }

  pub fn with__ref(mut self, _ref: String) -> PrBranchInfo {
    self._ref = Some(_ref);
    self
  }

  pub fn _ref(&self) -> Option<&String> {
    self._ref.as_ref()
  }

  pub fn reset__ref(&mut self) {
    self._ref = None;
  }

  pub fn set_repo(&mut self, repo: ::models::Repository) {
    self.repo = Some(repo);
  }

  pub fn with_repo(mut self, repo: ::models::Repository) -> PrBranchInfo {
    self.repo = Some(repo);
    self
  }

  pub fn repo(&self) -> Option<&::models::Repository> {
    self.repo.as_ref()
  }

  pub fn reset_repo(&mut self) {
    self.repo = None;
  }

  pub fn set_repo_id(&mut self, repo_id: i64) {
    self.repo_id = Some(repo_id);
  }

  pub fn with_repo_id(mut self, repo_id: i64) -> PrBranchInfo {
    self.repo_id = Some(repo_id);
    self
  }

  pub fn repo_id(&self) -> Option<&i64> {
    self.repo_id.as_ref()
  }

  pub fn reset_repo_id(&mut self) {
    self.repo_id = None;
  }

  pub fn set_sha(&mut self, sha: String) {
    self.sha = Some(sha);
  }

  pub fn with_sha(mut self, sha: String) -> PrBranchInfo {
    self.sha = Some(sha);
    self
  }

  pub fn sha(&self) -> Option<&String> {
    self.sha.as_ref()
  }

  pub fn reset_sha(&mut self) {
    self.sha = None;
  }

}



