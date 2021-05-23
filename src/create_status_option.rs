
/// CreateStatusOption holds the information needed to create a new CommitStatus for a Commit
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateStatusOption {
    pub context: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub target_url: Option<String>,
}

impl CreateStatusOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateStatusOptionBuilder {
        CreateStatusOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_create_status() -> CreateStatusOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingSha> {
        CreateStatusOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_sha: core::marker::PhantomData,
        }
    }
}

impl Into<CreateStatusOption> for CreateStatusOptionBuilder {
    fn into(self) -> CreateStatusOption {
        self.body
    }
}

impl Into<CreateStatusOption> for CreateStatusOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists> {
    fn into(self) -> CreateStatusOption {
        self.inner.body
    }
}

/// Builder for [`CreateStatusOption`](./struct.CreateStatusOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateStatusOptionBuilder {
    body: self::CreateStatusOption,
}

impl CreateStatusOptionBuilder {
    #[inline]
    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.body.context = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.body.state = Some(value.into());
        self
    }

    #[inline]
    pub fn target_url(mut self, value: impl Into<String>) -> Self {
        self.body.target_url = Some(value.into());
        self
    }
}

/// Builder created by [`CreateStatusOption::repo_create_status`](./struct.CreateStatusOption.html#method.repo_create_status) method for a `POST` operation associated with `CreateStatusOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateStatusOptionPostBuilder<Owner, Repo, Sha> {
    inner: CreateStatusOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_sha: core::marker::PhantomData<Sha>,
}

#[derive(Debug, Default, Clone)]
struct CreateStatusOptionPostBuilderContainer {
    body: self::CreateStatusOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_sha: Option<String>,
}

impl<Owner, Repo, Sha> CreateStatusOptionPostBuilder<Owner, Repo, Sha> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateStatusOptionPostBuilder<crate::generics::OwnerExists, Repo, Sha> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateStatusOptionPostBuilder<Owner, crate::generics::RepoExists, Sha> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// sha of the commit
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> CreateStatusOptionPostBuilder<Owner, Repo, crate::generics::ShaExists> {
        self.inner.param_sha = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.inner.body.context = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.inner.body.state = Some(value.into());
        self
    }

    #[inline]
    pub fn target_url(mut self, value: impl Into<String>) -> Self {
        self.inner.body.target_url = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateStatusOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists> {
    type Output = crate::commit_status::CommitStatus;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/statuses/{sha}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), sha=self.inner.param_sha.as_ref().expect("missing parameter sha?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::commit_status::CommitStatus, CreateStatusOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
