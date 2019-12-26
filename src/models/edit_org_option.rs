/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EditOrgOption : EditOrgOption options for editing an organization

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditOrgOption {
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "full_name")]
  full_name: Option<String>,
  #[serde(rename = "location")]
  location: Option<String>,
  #[serde(rename = "repo_admin_change_team_access")]
  repo_admin_change_team_access: Option<bool>,
  /// possible values are `public`, `limited` or `private`
  #[serde(rename = "visibility")]
  visibility: Option<String>,
  #[serde(rename = "website")]
  website: Option<String>
}

impl EditOrgOption {
  /// EditOrgOption options for editing an organization
  pub fn new() -> EditOrgOption {
    EditOrgOption {
      description: None,
      full_name: None,
      location: None,
      repo_admin_change_team_access: None,
      visibility: None,
      website: None
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> EditOrgOption {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_full_name(&mut self, full_name: String) {
    self.full_name = Some(full_name);
  }

  pub fn with_full_name(mut self, full_name: String) -> EditOrgOption {
    self.full_name = Some(full_name);
    self
  }

  pub fn full_name(&self) -> Option<&String> {
    self.full_name.as_ref()
  }

  pub fn reset_full_name(&mut self) {
    self.full_name = None;
  }

  pub fn set_location(&mut self, location: String) {
    self.location = Some(location);
  }

  pub fn with_location(mut self, location: String) -> EditOrgOption {
    self.location = Some(location);
    self
  }

  pub fn location(&self) -> Option<&String> {
    self.location.as_ref()
  }

  pub fn reset_location(&mut self) {
    self.location = None;
  }

  pub fn set_repo_admin_change_team_access(&mut self, repo_admin_change_team_access: bool) {
    self.repo_admin_change_team_access = Some(repo_admin_change_team_access);
  }

  pub fn with_repo_admin_change_team_access(mut self, repo_admin_change_team_access: bool) -> EditOrgOption {
    self.repo_admin_change_team_access = Some(repo_admin_change_team_access);
    self
  }

  pub fn repo_admin_change_team_access(&self) -> Option<&bool> {
    self.repo_admin_change_team_access.as_ref()
  }

  pub fn reset_repo_admin_change_team_access(&mut self) {
    self.repo_admin_change_team_access = None;
  }

  pub fn set_visibility(&mut self, visibility: String) {
    self.visibility = Some(visibility);
  }

  pub fn with_visibility(mut self, visibility: String) -> EditOrgOption {
    self.visibility = Some(visibility);
    self
  }

  pub fn visibility(&self) -> Option<&String> {
    self.visibility.as_ref()
  }

  pub fn reset_visibility(&mut self) {
    self.visibility = None;
  }

  pub fn set_website(&mut self, website: String) {
    self.website = Some(website);
  }

  pub fn with_website(mut self, website: String) -> EditOrgOption {
    self.website = Some(website);
    self
  }

  pub fn website(&self) -> Option<&String> {
    self.website.as_ref()
  }

  pub fn reset_website(&mut self) {
    self.website = None;
  }

}



