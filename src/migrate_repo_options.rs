
/// MigrateRepoOptions options for migrating repository's
/// this is used to interact with api v1
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MigrateRepoOptions {
    pub auth_password: Option<String>,
    pub auth_token: Option<String>,
    pub auth_username: Option<String>,
    pub clone_addr: String,
    pub description: Option<String>,
    pub issues: Option<bool>,
    pub labels: Option<bool>,
    pub milestones: Option<bool>,
    pub mirror: Option<bool>,
    pub mirror_interval: Option<String>,
    pub private: Option<bool>,
    pub pull_requests: Option<bool>,
    pub releases: Option<bool>,
    pub repo_name: String,
    /// Name of User or Organisation who will own Repo after migration
    pub repo_owner: Option<String>,
    pub service: Option<crate::migrate_repo_options::MigrateRepoOptionsService>,
    /// deprecated (only for backwards compatibility)
    pub uid: Option<i64>,
    pub wiki: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MigrateRepoOptionsService {
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitea")]
    Gitea,
    #[serde(rename = "gitlab")]
    Gitlab,
}
impl Default for MigrateRepoOptionsService {
    fn default() -> Self {
        MigrateRepoOptionsService::Git
    }
}

impl MigrateRepoOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> MigrateRepoOptionsBuilder<crate::generics::MissingCloneAddr, crate::generics::MissingRepoName> {
        MigrateRepoOptionsBuilder {
            body: Default::default(),
            _clone_addr: core::marker::PhantomData,
            _repo_name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_migrate() -> MigrateRepoOptionsPostBuilder<crate::generics::MissingCloneAddr, crate::generics::MissingRepoName> {
        MigrateRepoOptionsPostBuilder {
            body: Default::default(),
            _clone_addr: core::marker::PhantomData,
            _repo_name: core::marker::PhantomData,
        }
    }
}

impl Into<MigrateRepoOptions> for MigrateRepoOptionsBuilder<crate::generics::CloneAddrExists, crate::generics::RepoNameExists> {
    fn into(self) -> MigrateRepoOptions {
        self.body
    }
}

impl Into<MigrateRepoOptions> for MigrateRepoOptionsPostBuilder<crate::generics::CloneAddrExists, crate::generics::RepoNameExists> {
    fn into(self) -> MigrateRepoOptions {
        self.body
    }
}

/// Builder for [`MigrateRepoOptions`](./struct.MigrateRepoOptions.html) object.
#[derive(Debug, Clone)]
pub struct MigrateRepoOptionsBuilder<CloneAddr, RepoName> {
    body: self::MigrateRepoOptions,
    _clone_addr: core::marker::PhantomData<CloneAddr>,
    _repo_name: core::marker::PhantomData<RepoName>,
}

impl<CloneAddr, RepoName> MigrateRepoOptionsBuilder<CloneAddr, RepoName> {
    #[inline]
    pub fn auth_password(mut self, value: impl Into<String>) -> Self {
        self.body.auth_password = Some(value.into());
        self
    }

    #[inline]
    pub fn auth_token(mut self, value: impl Into<String>) -> Self {
        self.body.auth_token = Some(value.into());
        self
    }

    #[inline]
    pub fn auth_username(mut self, value: impl Into<String>) -> Self {
        self.body.auth_username = Some(value.into());
        self
    }

    #[inline]
    pub fn clone_addr(mut self, value: impl Into<String>) -> MigrateRepoOptionsBuilder<crate::generics::CloneAddrExists, RepoName> {
        self.body.clone_addr = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn issues(mut self, value: impl Into<bool>) -> Self {
        self.body.issues = Some(value.into());
        self
    }

    #[inline]
    pub fn labels(mut self, value: impl Into<bool>) -> Self {
        self.body.labels = Some(value.into());
        self
    }

    #[inline]
    pub fn milestones(mut self, value: impl Into<bool>) -> Self {
        self.body.milestones = Some(value.into());
        self
    }

    #[inline]
    pub fn mirror(mut self, value: impl Into<bool>) -> Self {
        self.body.mirror = Some(value.into());
        self
    }

    #[inline]
    pub fn mirror_interval(mut self, value: impl Into<String>) -> Self {
        self.body.mirror_interval = Some(value.into());
        self
    }

    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.body.private = Some(value.into());
        self
    }

    #[inline]
    pub fn pull_requests(mut self, value: impl Into<bool>) -> Self {
        self.body.pull_requests = Some(value.into());
        self
    }

    #[inline]
    pub fn releases(mut self, value: impl Into<bool>) -> Self {
        self.body.releases = Some(value.into());
        self
    }

    #[inline]
    pub fn repo_name(mut self, value: impl Into<String>) -> MigrateRepoOptionsBuilder<CloneAddr, crate::generics::RepoNameExists> {
        self.body.repo_name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Name of User or Organisation who will own Repo after migration
    #[inline]
    pub fn repo_owner(mut self, value: impl Into<String>) -> Self {
        self.body.repo_owner = Some(value.into());
        self
    }

    #[inline]
    pub fn service(mut self, value: crate::migrate_repo_options::MigrateRepoOptionsService) -> Self {
        self.body.service = Some(value.into());
        self
    }

    /// deprecated (only for backwards compatibility)
    #[inline]
    pub fn uid(mut self, value: impl Into<i64>) -> Self {
        self.body.uid = Some(value.into());
        self
    }

    #[inline]
    pub fn wiki(mut self, value: impl Into<bool>) -> Self {
        self.body.wiki = Some(value.into());
        self
    }
}

/// Builder created by [`MigrateRepoOptions::repo_migrate`](./struct.MigrateRepoOptions.html#method.repo_migrate) method for a `POST` operation associated with `MigrateRepoOptions`.
#[derive(Debug, Clone)]
pub struct MigrateRepoOptionsPostBuilder<CloneAddr, RepoName> {
    body: self::MigrateRepoOptions,
    _clone_addr: core::marker::PhantomData<CloneAddr>,
    _repo_name: core::marker::PhantomData<RepoName>,
}

impl<CloneAddr, RepoName> MigrateRepoOptionsPostBuilder<CloneAddr, RepoName> {
    #[inline]
    pub fn auth_password(mut self, value: impl Into<String>) -> Self {
        self.body.auth_password = Some(value.into());
        self
    }

    #[inline]
    pub fn auth_token(mut self, value: impl Into<String>) -> Self {
        self.body.auth_token = Some(value.into());
        self
    }

    #[inline]
    pub fn auth_username(mut self, value: impl Into<String>) -> Self {
        self.body.auth_username = Some(value.into());
        self
    }

    #[inline]
    pub fn clone_addr(mut self, value: impl Into<String>) -> MigrateRepoOptionsPostBuilder<crate::generics::CloneAddrExists, RepoName> {
        self.body.clone_addr = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn issues(mut self, value: impl Into<bool>) -> Self {
        self.body.issues = Some(value.into());
        self
    }

    #[inline]
    pub fn labels(mut self, value: impl Into<bool>) -> Self {
        self.body.labels = Some(value.into());
        self
    }

    #[inline]
    pub fn milestones(mut self, value: impl Into<bool>) -> Self {
        self.body.milestones = Some(value.into());
        self
    }

    #[inline]
    pub fn mirror(mut self, value: impl Into<bool>) -> Self {
        self.body.mirror = Some(value.into());
        self
    }

    #[inline]
    pub fn mirror_interval(mut self, value: impl Into<String>) -> Self {
        self.body.mirror_interval = Some(value.into());
        self
    }

    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.body.private = Some(value.into());
        self
    }

    #[inline]
    pub fn pull_requests(mut self, value: impl Into<bool>) -> Self {
        self.body.pull_requests = Some(value.into());
        self
    }

    #[inline]
    pub fn releases(mut self, value: impl Into<bool>) -> Self {
        self.body.releases = Some(value.into());
        self
    }

    #[inline]
    pub fn repo_name(mut self, value: impl Into<String>) -> MigrateRepoOptionsPostBuilder<CloneAddr, crate::generics::RepoNameExists> {
        self.body.repo_name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Name of User or Organisation who will own Repo after migration
    #[inline]
    pub fn repo_owner(mut self, value: impl Into<String>) -> Self {
        self.body.repo_owner = Some(value.into());
        self
    }

    #[inline]
    pub fn service(mut self, value: crate::migrate_repo_options::MigrateRepoOptionsService) -> Self {
        self.body.service = Some(value.into());
        self
    }

    /// deprecated (only for backwards compatibility)
    #[inline]
    pub fn uid(mut self, value: impl Into<i64>) -> Self {
        self.body.uid = Some(value.into());
        self
    }

    #[inline]
    pub fn wiki(mut self, value: impl Into<bool>) -> Self {
        self.body.wiki = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MigrateRepoOptionsPostBuilder<crate::generics::CloneAddrExists, crate::generics::RepoNameExists> {
    type Output = crate::repository::Repository;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/repos/migrate".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}

impl crate::client::ResponseWrapper<crate::repository::Repository, MigrateRepoOptionsPostBuilder<crate::generics::CloneAddrExists, crate::generics::RepoNameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

