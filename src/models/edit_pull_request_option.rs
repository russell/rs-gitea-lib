/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EditPullRequestOption : EditPullRequestOption options when modify pull request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditPullRequestOption {
  #[serde(rename = "assignee")]
  assignee: Option<String>,
  #[serde(rename = "assignees")]
  assignees: Option<Vec<String>>,
  #[serde(rename = "body")]
  body: Option<String>,
  #[serde(rename = "due_date")]
  due_date: Option<String>,
  #[serde(rename = "labels")]
  labels: Option<Vec<i64>>,
  #[serde(rename = "milestone")]
  milestone: Option<i64>,
  #[serde(rename = "state")]
  state: Option<String>,
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "unset_due_date")]
  unset_due_date: Option<bool>
}

impl EditPullRequestOption {
  /// EditPullRequestOption options when modify pull request
  pub fn new() -> EditPullRequestOption {
    EditPullRequestOption {
      assignee: None,
      assignees: None,
      body: None,
      due_date: None,
      labels: None,
      milestone: None,
      state: None,
      title: None,
      unset_due_date: None
    }
  }

  pub fn set_assignee(&mut self, assignee: String) {
    self.assignee = Some(assignee);
  }

  pub fn with_assignee(mut self, assignee: String) -> EditPullRequestOption {
    self.assignee = Some(assignee);
    self
  }

  pub fn assignee(&self) -> Option<&String> {
    self.assignee.as_ref()
  }

  pub fn reset_assignee(&mut self) {
    self.assignee = None;
  }

  pub fn set_assignees(&mut self, assignees: Vec<String>) {
    self.assignees = Some(assignees);
  }

  pub fn with_assignees(mut self, assignees: Vec<String>) -> EditPullRequestOption {
    self.assignees = Some(assignees);
    self
  }

  pub fn assignees(&self) -> Option<&Vec<String>> {
    self.assignees.as_ref()
  }

  pub fn reset_assignees(&mut self) {
    self.assignees = None;
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> EditPullRequestOption {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_due_date(&mut self, due_date: String) {
    self.due_date = Some(due_date);
  }

  pub fn with_due_date(mut self, due_date: String) -> EditPullRequestOption {
    self.due_date = Some(due_date);
    self
  }

  pub fn due_date(&self) -> Option<&String> {
    self.due_date.as_ref()
  }

  pub fn reset_due_date(&mut self) {
    self.due_date = None;
  }

  pub fn set_labels(&mut self, labels: Vec<i64>) {
    self.labels = Some(labels);
  }

  pub fn with_labels(mut self, labels: Vec<i64>) -> EditPullRequestOption {
    self.labels = Some(labels);
    self
  }

  pub fn labels(&self) -> Option<&Vec<i64>> {
    self.labels.as_ref()
  }

  pub fn reset_labels(&mut self) {
    self.labels = None;
  }

  pub fn set_milestone(&mut self, milestone: i64) {
    self.milestone = Some(milestone);
  }

  pub fn with_milestone(mut self, milestone: i64) -> EditPullRequestOption {
    self.milestone = Some(milestone);
    self
  }

  pub fn milestone(&self) -> Option<&i64> {
    self.milestone.as_ref()
  }

  pub fn reset_milestone(&mut self) {
    self.milestone = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> EditPullRequestOption {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> EditPullRequestOption {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_unset_due_date(&mut self, unset_due_date: bool) {
    self.unset_due_date = Some(unset_due_date);
  }

  pub fn with_unset_due_date(mut self, unset_due_date: bool) -> EditPullRequestOption {
    self.unset_due_date = Some(unset_due_date);
    self
  }

  pub fn unset_due_date(&self) -> Option<&bool> {
    self.unset_due_date.as_ref()
  }

  pub fn reset_unset_due_date(&mut self) {
    self.unset_due_date = None;
  }

}



