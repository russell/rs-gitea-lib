/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Release : Release represents a repository release

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
  #[serde(rename = "assets")]
  assets: Option<Vec<::models::Attachment>>,
  #[serde(rename = "author")]
  author: Option<::models::User>,
  #[serde(rename = "body")]
  body: Option<String>,
  #[serde(rename = "created_at")]
  created_at: Option<String>,
  #[serde(rename = "draft")]
  draft: Option<bool>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "prerelease")]
  prerelease: Option<bool>,
  #[serde(rename = "published_at")]
  published_at: Option<String>,
  #[serde(rename = "tag_name")]
  tag_name: Option<String>,
  #[serde(rename = "tarball_url")]
  tarball_url: Option<String>,
  #[serde(rename = "target_commitish")]
  target_commitish: Option<String>,
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "zipball_url")]
  zipball_url: Option<String>
}

impl Release {
  /// Release represents a repository release
  pub fn new() -> Release {
    Release {
      assets: None,
      author: None,
      body: None,
      created_at: None,
      draft: None,
      id: None,
      name: None,
      prerelease: None,
      published_at: None,
      tag_name: None,
      tarball_url: None,
      target_commitish: None,
      url: None,
      zipball_url: None
    }
  }

  pub fn set_assets(&mut self, assets: Vec<::models::Attachment>) {
    self.assets = Some(assets);
  }

  pub fn with_assets(mut self, assets: Vec<::models::Attachment>) -> Release {
    self.assets = Some(assets);
    self
  }

  pub fn assets(&self) -> Option<&Vec<::models::Attachment>> {
    self.assets.as_ref()
  }

  pub fn reset_assets(&mut self) {
    self.assets = None;
  }

  pub fn set_author(&mut self, author: ::models::User) {
    self.author = Some(author);
  }

  pub fn with_author(mut self, author: ::models::User) -> Release {
    self.author = Some(author);
    self
  }

  pub fn author(&self) -> Option<&::models::User> {
    self.author.as_ref()
  }

  pub fn reset_author(&mut self) {
    self.author = None;
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> Release {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> Release {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_draft(&mut self, draft: bool) {
    self.draft = Some(draft);
  }

  pub fn with_draft(mut self, draft: bool) -> Release {
    self.draft = Some(draft);
    self
  }

  pub fn draft(&self) -> Option<&bool> {
    self.draft.as_ref()
  }

  pub fn reset_draft(&mut self) {
    self.draft = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Release {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Release {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_prerelease(&mut self, prerelease: bool) {
    self.prerelease = Some(prerelease);
  }

  pub fn with_prerelease(mut self, prerelease: bool) -> Release {
    self.prerelease = Some(prerelease);
    self
  }

  pub fn prerelease(&self) -> Option<&bool> {
    self.prerelease.as_ref()
  }

  pub fn reset_prerelease(&mut self) {
    self.prerelease = None;
  }

  pub fn set_published_at(&mut self, published_at: String) {
    self.published_at = Some(published_at);
  }

  pub fn with_published_at(mut self, published_at: String) -> Release {
    self.published_at = Some(published_at);
    self
  }

  pub fn published_at(&self) -> Option<&String> {
    self.published_at.as_ref()
  }

  pub fn reset_published_at(&mut self) {
    self.published_at = None;
  }

  pub fn set_tag_name(&mut self, tag_name: String) {
    self.tag_name = Some(tag_name);
  }

  pub fn with_tag_name(mut self, tag_name: String) -> Release {
    self.tag_name = Some(tag_name);
    self
  }

  pub fn tag_name(&self) -> Option<&String> {
    self.tag_name.as_ref()
  }

  pub fn reset_tag_name(&mut self) {
    self.tag_name = None;
  }

  pub fn set_tarball_url(&mut self, tarball_url: String) {
    self.tarball_url = Some(tarball_url);
  }

  pub fn with_tarball_url(mut self, tarball_url: String) -> Release {
    self.tarball_url = Some(tarball_url);
    self
  }

  pub fn tarball_url(&self) -> Option<&String> {
    self.tarball_url.as_ref()
  }

  pub fn reset_tarball_url(&mut self) {
    self.tarball_url = None;
  }

  pub fn set_target_commitish(&mut self, target_commitish: String) {
    self.target_commitish = Some(target_commitish);
  }

  pub fn with_target_commitish(mut self, target_commitish: String) -> Release {
    self.target_commitish = Some(target_commitish);
    self
  }

  pub fn target_commitish(&self) -> Option<&String> {
    self.target_commitish.as_ref()
  }

  pub fn reset_target_commitish(&mut self) {
    self.target_commitish = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Release {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_zipball_url(&mut self, zipball_url: String) {
    self.zipball_url = Some(zipball_url);
  }

  pub fn with_zipball_url(mut self, zipball_url: String) -> Release {
    self.zipball_url = Some(zipball_url);
    self
  }

  pub fn zipball_url(&self) -> Option<&String> {
    self.zipball_url.as_ref()
  }

  pub fn reset_zipball_url(&mut self) {
    self.zipball_url = None;
  }

}


