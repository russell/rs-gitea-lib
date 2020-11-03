
/// Namespace for operations that cannot be added to any other modules.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Miscellaneous {}

impl Miscellaneous {
    #[inline]
    pub fn admin_unadopted_list() -> MiscellaneousGetBuilder {
        MiscellaneousGetBuilder {
            param_page: None,
            param_limit: None,
            param_pattern: None,
        }
    }

    #[inline]
    pub fn render_markdown_raw() -> MiscellaneousPostBuilder1 {
        MiscellaneousPostBuilder1
    }

    #[inline]
    pub fn repo_download_pull_diff() -> MiscellaneousGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        MiscellaneousGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_download_pull_patch() -> MiscellaneousGetBuilder3<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        MiscellaneousGetBuilder3 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_signing_key() -> MiscellaneousGetBuilder4<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        MiscellaneousGetBuilder4 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn get_signing_key() -> MiscellaneousGetBuilder5 {
        MiscellaneousGetBuilder5
    }
}

/// Builder created by [`Miscellaneous::admin_unadopted_list`](./struct.Miscellaneous.html#method.admin_unadopted_list) method for a `GET` operation associated with `Miscellaneous`.
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder {
    param_page: Option<i64>,
    param_limit: Option<i64>,
    param_pattern: Option<String>,
}

impl MiscellaneousGetBuilder {
    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.param_limit = Some(value.into());
        self
    }

    /// pattern of repositories to search for
    #[inline]
    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.param_pattern = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder {
    type Output = Vec<String>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/admin/unadopted".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string)),
            ("pattern", self.param_pattern.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

impl crate::client::ResponseWrapper<Vec<String>, MiscellaneousGetBuilder> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`Miscellaneous::render_markdown_raw`](./struct.Miscellaneous.html#method.render_markdown_raw) method for a `POST` operation associated with `Miscellaneous`.
#[derive(Debug, Clone)]
pub struct MiscellaneousPostBuilder1;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousPostBuilder1 {
    type Output = String;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/markdown/raw".into()
    }
}

impl crate::client::ResponseWrapper<String, MiscellaneousPostBuilder1> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`Miscellaneous::repo_download_pull_diff`](./struct.Miscellaneous.html#method.repo_download_pull_diff) method for a `GET` operation associated with `Miscellaneous`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder2<Owner, Repo, Index> {
    inner: MiscellaneousGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct MiscellaneousGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> MiscellaneousGetBuilder2<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> MiscellaneousGetBuilder2<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> MiscellaneousGetBuilder2<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request to get
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> MiscellaneousGetBuilder2<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = String;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}.diff", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }
}

/// Builder created by [`Miscellaneous::repo_download_pull_patch`](./struct.Miscellaneous.html#method.repo_download_pull_patch) method for a `GET` operation associated with `Miscellaneous`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder3<Owner, Repo, Index> {
    inner: MiscellaneousGetBuilder3Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct MiscellaneousGetBuilder3Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> MiscellaneousGetBuilder3<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> MiscellaneousGetBuilder3<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> MiscellaneousGetBuilder3<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request to get
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> MiscellaneousGetBuilder3<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder3<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = String;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}.patch", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }
}

/// Builder created by [`Miscellaneous::repo_signing_key`](./struct.Miscellaneous.html#method.repo_signing_key) method for a `GET` operation associated with `Miscellaneous`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder4<Owner, Repo> {
    inner: MiscellaneousGetBuilder4Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct MiscellaneousGetBuilder4Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> MiscellaneousGetBuilder4<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> MiscellaneousGetBuilder4<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> MiscellaneousGetBuilder4<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder4<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = String;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/signing-key.gpg", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`Miscellaneous::get_signing_key`](./struct.Miscellaneous.html#method.get_signing_key) method for a `GET` operation associated with `Miscellaneous`.
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder5;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder5 {
    type Output = String;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/signing-key.gpg".into()
    }
}
