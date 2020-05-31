
/// GitTreeResponse returns a git tree
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitTreeResponse {
    pub page: Option<i64>,
    pub sha: Option<String>,
    pub total_count: Option<i64>,
    pub tree: Option<Vec<crate::git_entry::GitEntry>>,
    pub truncated: Option<bool>,
    pub url: Option<String>,
}

impl GitTreeResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GitTreeResponseBuilder {
        GitTreeResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get_tree() -> GitTreeResponseGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingSha> {
        GitTreeResponseGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_sha: core::marker::PhantomData,
        }
    }
}

impl Into<GitTreeResponse> for GitTreeResponseBuilder {
    fn into(self) -> GitTreeResponse {
        self.body
    }
}

/// Builder for [`GitTreeResponse`](./struct.GitTreeResponse.html) object.
#[derive(Debug, Clone)]
pub struct GitTreeResponseBuilder {
    body: self::GitTreeResponse,
}

impl GitTreeResponseBuilder {
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.body.page = Some(value.into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }

    #[inline]
    pub fn total_count(mut self, value: impl Into<i64>) -> Self {
        self.body.total_count = Some(value.into());
        self
    }

    #[inline]
    pub fn tree(mut self, value: impl Iterator<Item = crate::git_entry::GitEntry>) -> Self {
        self.body.tree = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn truncated(mut self, value: impl Into<bool>) -> Self {
        self.body.truncated = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`GitTreeResponse::get_tree`](./struct.GitTreeResponse.html#method.get_tree) method for a `GET` operation associated with `GitTreeResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GitTreeResponseGetBuilder<Owner, Repo, Sha> {
    inner: GitTreeResponseGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_sha: core::marker::PhantomData<Sha>,
}

#[derive(Debug, Default, Clone)]
struct GitTreeResponseGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_sha: Option<String>,
    param_recursive: Option<bool>,
    param_page: Option<i64>,
    param_per_page: Option<i64>,
}

impl<Owner, Repo, Sha> GitTreeResponseGetBuilder<Owner, Repo, Sha> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> GitTreeResponseGetBuilder<crate::generics::OwnerExists, Repo, Sha> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> GitTreeResponseGetBuilder<Owner, crate::generics::RepoExists, Sha> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// sha of the commit
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> GitTreeResponseGetBuilder<Owner, Repo, crate::generics::ShaExists> {
        self.inner.param_sha = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// show all directories and files
    #[inline]
    pub fn recursive(mut self, value: impl Into<bool>) -> Self {
        self.inner.param_recursive = Some(value.into());
        self
    }

    /// page number; the 'truncated' field in the response will be true if there are still more items after this page, false if the last page
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// number of items per page; default is 1000 or what is set in app.ini as DEFAULT_GIT_TREES_PER_PAGE
    #[inline]
    pub fn per_page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_per_page = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GitTreeResponseGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists> {
    type Output = GitTreeResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/git/trees/{sha}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), sha=self.inner.param_sha.as_ref().expect("missing parameter sha?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("recursive", self.inner.param_recursive.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("per_page", self.inner.param_per_page.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

impl crate::client::ResponseWrapper<GitTreeResponse, GitTreeResponseGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
