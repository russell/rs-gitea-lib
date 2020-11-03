
/// IssueLabelsOption a collection of labels
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssueLabelsOption {
    /// list of label IDs
    pub labels: Option<Vec<i64>>,
}

impl IssueLabelsOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> IssueLabelsOptionBuilder {
        IssueLabelsOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_replace_labels() -> IssueLabelsOptionPutBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        IssueLabelsOptionPutBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_add_label() -> IssueLabelsOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        IssueLabelsOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<IssueLabelsOption> for IssueLabelsOptionBuilder {
    fn into(self) -> IssueLabelsOption {
        self.body
    }
}

impl Into<IssueLabelsOption> for IssueLabelsOptionPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> IssueLabelsOption {
        self.inner.body
    }
}

impl Into<IssueLabelsOption> for IssueLabelsOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> IssueLabelsOption {
        self.inner.body
    }
}

/// Builder for [`IssueLabelsOption`](./struct.IssueLabelsOption.html) object.
#[derive(Debug, Clone)]
pub struct IssueLabelsOptionBuilder {
    body: self::IssueLabelsOption,
}

impl IssueLabelsOptionBuilder {
    /// list of label IDs
    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`IssueLabelsOption::issue_replace_labels`](./struct.IssueLabelsOption.html#method.issue_replace_labels) method for a `PUT` operation associated with `IssueLabelsOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct IssueLabelsOptionPutBuilder<Owner, Repo, Index> {
    inner: IssueLabelsOptionPutBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct IssueLabelsOptionPutBuilderContainer {
    body: self::IssueLabelsOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> IssueLabelsOptionPutBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> IssueLabelsOptionPutBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> IssueLabelsOptionPutBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> IssueLabelsOptionPutBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// list of label IDs
    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.inner.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for IssueLabelsOptionPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<crate::label::Label>;

    const METHOD: http::Method = http::Method::PUT;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/labels", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<Vec<crate::label::Label>, IssueLabelsOptionPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`IssueLabelsOption::issue_add_label`](./struct.IssueLabelsOption.html#method.issue_add_label) method for a `POST` operation associated with `IssueLabelsOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct IssueLabelsOptionPostBuilder<Owner, Repo, Index> {
    inner: IssueLabelsOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct IssueLabelsOptionPostBuilderContainer {
    body: self::IssueLabelsOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> IssueLabelsOptionPostBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> IssueLabelsOptionPostBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> IssueLabelsOptionPostBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> IssueLabelsOptionPostBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// list of label IDs
    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.inner.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for IssueLabelsOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<crate::label::Label>;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/labels", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<Vec<crate::label::Label>, IssueLabelsOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
