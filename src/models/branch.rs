/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Branch : Branch represents a repository branch

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
  #[serde(rename = "commit")]
  commit: Option<::models::PayloadCommit>,
  #[serde(rename = "enable_status_check")]
  enable_status_check: Option<bool>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "protected")]
  protected: Option<bool>,
  #[serde(rename = "required_approvals")]
  required_approvals: Option<i64>,
  #[serde(rename = "status_check_contexts")]
  status_check_contexts: Option<Vec<String>>,
  #[serde(rename = "user_can_merge")]
  user_can_merge: Option<bool>,
  #[serde(rename = "user_can_push")]
  user_can_push: Option<bool>
}

impl Branch {
  /// Branch represents a repository branch
  pub fn new() -> Branch {
    Branch {
      commit: None,
      enable_status_check: None,
      name: None,
      protected: None,
      required_approvals: None,
      status_check_contexts: None,
      user_can_merge: None,
      user_can_push: None
    }
  }

  pub fn set_commit(&mut self, commit: ::models::PayloadCommit) {
    self.commit = Some(commit);
  }

  pub fn with_commit(mut self, commit: ::models::PayloadCommit) -> Branch {
    self.commit = Some(commit);
    self
  }

  pub fn commit(&self) -> Option<&::models::PayloadCommit> {
    self.commit.as_ref()
  }

  pub fn reset_commit(&mut self) {
    self.commit = None;
  }

  pub fn set_enable_status_check(&mut self, enable_status_check: bool) {
    self.enable_status_check = Some(enable_status_check);
  }

  pub fn with_enable_status_check(mut self, enable_status_check: bool) -> Branch {
    self.enable_status_check = Some(enable_status_check);
    self
  }

  pub fn enable_status_check(&self) -> Option<&bool> {
    self.enable_status_check.as_ref()
  }

  pub fn reset_enable_status_check(&mut self) {
    self.enable_status_check = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Branch {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_protected(&mut self, protected: bool) {
    self.protected = Some(protected);
  }

  pub fn with_protected(mut self, protected: bool) -> Branch {
    self.protected = Some(protected);
    self
  }

  pub fn protected(&self) -> Option<&bool> {
    self.protected.as_ref()
  }

  pub fn reset_protected(&mut self) {
    self.protected = None;
  }

  pub fn set_required_approvals(&mut self, required_approvals: i64) {
    self.required_approvals = Some(required_approvals);
  }

  pub fn with_required_approvals(mut self, required_approvals: i64) -> Branch {
    self.required_approvals = Some(required_approvals);
    self
  }

  pub fn required_approvals(&self) -> Option<&i64> {
    self.required_approvals.as_ref()
  }

  pub fn reset_required_approvals(&mut self) {
    self.required_approvals = None;
  }

  pub fn set_status_check_contexts(&mut self, status_check_contexts: Vec<String>) {
    self.status_check_contexts = Some(status_check_contexts);
  }

  pub fn with_status_check_contexts(mut self, status_check_contexts: Vec<String>) -> Branch {
    self.status_check_contexts = Some(status_check_contexts);
    self
  }

  pub fn status_check_contexts(&self) -> Option<&Vec<String>> {
    self.status_check_contexts.as_ref()
  }

  pub fn reset_status_check_contexts(&mut self) {
    self.status_check_contexts = None;
  }

  pub fn set_user_can_merge(&mut self, user_can_merge: bool) {
    self.user_can_merge = Some(user_can_merge);
  }

  pub fn with_user_can_merge(mut self, user_can_merge: bool) -> Branch {
    self.user_can_merge = Some(user_can_merge);
    self
  }

  pub fn user_can_merge(&self) -> Option<&bool> {
    self.user_can_merge.as_ref()
  }

  pub fn reset_user_can_merge(&mut self) {
    self.user_can_merge = None;
  }

  pub fn set_user_can_push(&mut self, user_can_push: bool) {
    self.user_can_push = Some(user_can_push);
  }

  pub fn with_user_can_push(mut self, user_can_push: bool) -> Branch {
    self.user_can_push = Some(user_can_push);
    self
  }

  pub fn user_can_push(&self) -> Option<&bool> {
    self.user_can_push.as_ref()
  }

  pub fn reset_user_can_push(&mut self) {
    self.user_can_push = None;
  }

}



