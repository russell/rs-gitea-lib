
/// GitBlobResponse represents a git blob
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitBlobResponse {
    pub content: Option<String>,
    pub encoding: Option<String>,
    pub sha: Option<String>,
    pub size: Option<i64>,
    pub url: Option<String>,
}

impl GitBlobResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GitBlobResponseBuilder {
        GitBlobResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get_blob() -> GitBlobResponseGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingSha> {
        GitBlobResponseGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_sha: core::marker::PhantomData,
        }
    }
}

impl Into<GitBlobResponse> for GitBlobResponseBuilder {
    fn into(self) -> GitBlobResponse {
        self.body
    }
}

/// Builder for [`GitBlobResponse`](./struct.GitBlobResponse.html) object.
#[derive(Debug, Clone)]
pub struct GitBlobResponseBuilder {
    body: self::GitBlobResponse,
}

impl GitBlobResponseBuilder {
    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.body.content = Some(value.into());
        self
    }

    #[inline]
    pub fn encoding(mut self, value: impl Into<String>) -> Self {
        self.body.encoding = Some(value.into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }

    #[inline]
    pub fn size(mut self, value: impl Into<i64>) -> Self {
        self.body.size = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`GitBlobResponse::get_blob`](./struct.GitBlobResponse.html#method.get_blob) method for a `GET` operation associated with `GitBlobResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GitBlobResponseGetBuilder<Owner, Repo, Sha> {
    inner: GitBlobResponseGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_sha: core::marker::PhantomData<Sha>,
}

#[derive(Debug, Default, Clone)]
struct GitBlobResponseGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_sha: Option<String>,
}

impl<Owner, Repo, Sha> GitBlobResponseGetBuilder<Owner, Repo, Sha> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> GitBlobResponseGetBuilder<crate::generics::OwnerExists, Repo, Sha> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> GitBlobResponseGetBuilder<Owner, crate::generics::RepoExists, Sha> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// sha of the commit
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> GitBlobResponseGetBuilder<Owner, Repo, crate::generics::ShaExists> {
        self.inner.param_sha = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GitBlobResponseGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists> {
    type Output = GitBlobResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/git/blobs/{sha}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), sha=self.inner.param_sha.as_ref().expect("missing parameter sha?")).into()
    }
}
