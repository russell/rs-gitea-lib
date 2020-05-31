
/// ContentsResponse contains information about a repo's entry's (dir, file, symlink, submodule) metadata and content
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ContentsResponse {
    #[serde(rename = "_links")]
    pub links: Option<crate::file_links_response::FileLinksResponse>,
    /// `content` is populated when `type` is `file`, otherwise null
    pub content: Option<String>,
    pub download_url: Option<String>,
    /// `encoding` is populated when `type` is `file`, otherwise null
    pub encoding: Option<String>,
    pub git_url: Option<String>,
    pub html_url: Option<String>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub sha: Option<String>,
    pub size: Option<i64>,
    /// `submodule_git_url` is populated when `type` is `submodule`, otherwise null
    pub submodule_git_url: Option<String>,
    /// `target` is populated when `type` is `symlink`, otherwise null
    pub target: Option<String>,
    /// `type` will be `file`, `dir`, `symlink`, or `submodule`
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
}

impl ContentsResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ContentsResponseBuilder {
        ContentsResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_get_contents_list() -> ContentsResponseGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        ContentsResponseGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_contents() -> ContentsResponseGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingFilepath> {
        ContentsResponseGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_filepath: core::marker::PhantomData,
        }
    }
}

impl Into<ContentsResponse> for ContentsResponseBuilder {
    fn into(self) -> ContentsResponse {
        self.body
    }
}

/// Builder for [`ContentsResponse`](./struct.ContentsResponse.html) object.
#[derive(Debug, Clone)]
pub struct ContentsResponseBuilder {
    body: self::ContentsResponse,
}

impl ContentsResponseBuilder {
    #[inline]
    pub fn links(mut self, value: crate::file_links_response::FileLinksResponse) -> Self {
        self.body.links = Some(value.into());
        self
    }

    /// `content` is populated when `type` is `file`, otherwise null
    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.body.content = Some(value.into());
        self
    }

    #[inline]
    pub fn download_url(mut self, value: impl Into<String>) -> Self {
        self.body.download_url = Some(value.into());
        self
    }

    /// `encoding` is populated when `type` is `file`, otherwise null
    #[inline]
    pub fn encoding(mut self, value: impl Into<String>) -> Self {
        self.body.encoding = Some(value.into());
        self
    }

    #[inline]
    pub fn git_url(mut self, value: impl Into<String>) -> Self {
        self.body.git_url = Some(value.into());
        self
    }

    #[inline]
    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.body.html_url = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.body.path = Some(value.into());
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

    /// `submodule_git_url` is populated when `type` is `submodule`, otherwise null
    #[inline]
    pub fn submodule_git_url(mut self, value: impl Into<String>) -> Self {
        self.body.submodule_git_url = Some(value.into());
        self
    }

    /// `target` is populated when `type` is `symlink`, otherwise null
    #[inline]
    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.body.target = Some(value.into());
        self
    }

    /// `type` will be `file`, `dir`, `symlink`, or `submodule`
    #[inline]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.body.type_ = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`ContentsResponse::repo_get_contents_list`](./struct.ContentsResponse.html#method.repo_get_contents_list) method for a `GET` operation associated with `ContentsResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ContentsResponseGetBuilder<Owner, Repo> {
    inner: ContentsResponseGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct ContentsResponseGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_ref: Option<String>,
}

impl<Owner, Repo> ContentsResponseGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> ContentsResponseGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> ContentsResponseGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// The name of the commit/branch/tag. Default the repository’s default branch (usually master)
    #[inline]
    pub fn ref_(mut self, value: impl Into<String>) -> Self {
        self.inner.param_ref = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ContentsResponseGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<ContentsResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/contents", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("ref", self.inner.param_ref.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`ContentsResponse::repo_get_contents`](./struct.ContentsResponse.html#method.repo_get_contents) method for a `GET` operation associated with `ContentsResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ContentsResponseGetBuilder1<Owner, Repo, Filepath> {
    inner: ContentsResponseGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_filepath: core::marker::PhantomData<Filepath>,
}

#[derive(Debug, Default, Clone)]
struct ContentsResponseGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_filepath: Option<String>,
    param_ref: Option<String>,
}

impl<Owner, Repo, Filepath> ContentsResponseGetBuilder1<Owner, Repo, Filepath> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> ContentsResponseGetBuilder1<crate::generics::OwnerExists, Repo, Filepath> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> ContentsResponseGetBuilder1<Owner, crate::generics::RepoExists, Filepath> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// path of the dir, file, symlink or submodule in the repo
    #[inline]
    pub fn filepath(mut self, value: impl Into<String>) -> ContentsResponseGetBuilder1<Owner, Repo, crate::generics::FilepathExists> {
        self.inner.param_filepath = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// The name of the commit/branch/tag. Default the repository’s default branch (usually master)
    #[inline]
    pub fn ref_(mut self, value: impl Into<String>) -> Self {
        self.inner.param_ref = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ContentsResponseGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::FilepathExists> {
    type Output = ContentsResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/contents/{filepath}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), filepath=self.inner.param_filepath.as_ref().expect("missing parameter filepath?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("ref", self.inner.param_ref.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
