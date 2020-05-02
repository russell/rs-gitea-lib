
/// MigrateRepoForm form for migrating repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MigrateRepoForm {
    pub auth_password: Option<String>,
    pub auth_username: Option<String>,
    pub clone_addr: String,
    pub description: Option<String>,
    pub issues: Option<bool>,
    pub labels: Option<bool>,
    pub milestones: Option<bool>,
    pub mirror: Option<bool>,
    pub private: Option<bool>,
    pub pull_requests: Option<bool>,
    pub releases: Option<bool>,
    pub repo_name: String,
    pub uid: i64,
    pub wiki: Option<bool>,
}

impl MigrateRepoForm {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> MigrateRepoFormBuilder<crate::generics::MissingCloneAddr, crate::generics::MissingRepoName, crate::generics::MissingUid> {
        MigrateRepoFormBuilder {
            body: Default::default(),
            _clone_addr: core::marker::PhantomData,
            _repo_name: core::marker::PhantomData,
            _uid: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_migrate() -> MigrateRepoFormPostBuilder<crate::generics::MissingCloneAddr, crate::generics::MissingRepoName, crate::generics::MissingUid> {
        MigrateRepoFormPostBuilder {
            body: Default::default(),
            _clone_addr: core::marker::PhantomData,
            _repo_name: core::marker::PhantomData,
            _uid: core::marker::PhantomData,
        }
    }
}

impl Into<MigrateRepoForm> for MigrateRepoFormBuilder<crate::generics::CloneAddrExists, crate::generics::RepoNameExists, crate::generics::UidExists> {
    fn into(self) -> MigrateRepoForm {
        self.body
    }
}

impl Into<MigrateRepoForm> for MigrateRepoFormPostBuilder<crate::generics::CloneAddrExists, crate::generics::RepoNameExists, crate::generics::UidExists> {
    fn into(self) -> MigrateRepoForm {
        self.body
    }
}

/// Builder for [`MigrateRepoForm`](./struct.MigrateRepoForm.html) object.
#[derive(Debug, Clone)]
pub struct MigrateRepoFormBuilder<CloneAddr, RepoName, Uid> {
    body: self::MigrateRepoForm,
    _clone_addr: core::marker::PhantomData<CloneAddr>,
    _repo_name: core::marker::PhantomData<RepoName>,
    _uid: core::marker::PhantomData<Uid>,
}

impl<CloneAddr, RepoName, Uid> MigrateRepoFormBuilder<CloneAddr, RepoName, Uid> {
    #[inline]
    pub fn auth_password(mut self, value: impl Into<String>) -> Self {
        self.body.auth_password = Some(value.into());
        self
    }

    #[inline]
    pub fn auth_username(mut self, value: impl Into<String>) -> Self {
        self.body.auth_username = Some(value.into());
        self
    }

    #[inline]
    pub fn clone_addr(mut self, value: impl Into<String>) -> MigrateRepoFormBuilder<crate::generics::CloneAddrExists, RepoName, Uid> {
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
    pub fn repo_name(mut self, value: impl Into<String>) -> MigrateRepoFormBuilder<CloneAddr, crate::generics::RepoNameExists, Uid> {
        self.body.repo_name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn uid(mut self, value: impl Into<i64>) -> MigrateRepoFormBuilder<CloneAddr, RepoName, crate::generics::UidExists> {
        self.body.uid = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn wiki(mut self, value: impl Into<bool>) -> Self {
        self.body.wiki = Some(value.into());
        self
    }
}

/// Builder created by [`MigrateRepoForm::repo_migrate`](./struct.MigrateRepoForm.html#method.repo_migrate) method for a `POST` operation associated with `MigrateRepoForm`.
#[derive(Debug, Clone)]
pub struct MigrateRepoFormPostBuilder<CloneAddr, RepoName, Uid> {
    body: self::MigrateRepoForm,
    _clone_addr: core::marker::PhantomData<CloneAddr>,
    _repo_name: core::marker::PhantomData<RepoName>,
    _uid: core::marker::PhantomData<Uid>,
}

impl<CloneAddr, RepoName, Uid> MigrateRepoFormPostBuilder<CloneAddr, RepoName, Uid> {
    #[inline]
    pub fn auth_password(mut self, value: impl Into<String>) -> Self {
        self.body.auth_password = Some(value.into());
        self
    }

    #[inline]
    pub fn auth_username(mut self, value: impl Into<String>) -> Self {
        self.body.auth_username = Some(value.into());
        self
    }

    #[inline]
    pub fn clone_addr(mut self, value: impl Into<String>) -> MigrateRepoFormPostBuilder<crate::generics::CloneAddrExists, RepoName, Uid> {
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
    pub fn repo_name(mut self, value: impl Into<String>) -> MigrateRepoFormPostBuilder<CloneAddr, crate::generics::RepoNameExists, Uid> {
        self.body.repo_name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn uid(mut self, value: impl Into<i64>) -> MigrateRepoFormPostBuilder<CloneAddr, RepoName, crate::generics::UidExists> {
        self.body.uid = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn wiki(mut self, value: impl Into<bool>) -> Self {
        self.body.wiki = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MigrateRepoFormPostBuilder<crate::generics::CloneAddrExists, crate::generics::RepoNameExists, crate::generics::UidExists> {
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
