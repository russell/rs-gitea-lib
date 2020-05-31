
/// CreatePullRequestOption options when creating a pull request
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreatePullRequestOption {
    pub assignee: Option<String>,
    pub assignees: Option<Vec<String>>,
    pub base: Option<String>,
    pub body: Option<String>,
    pub due_date: Option<String>,
    pub head: Option<String>,
    pub labels: Option<Vec<i64>>,
    pub milestone: Option<i64>,
    pub title: Option<String>,
}

impl CreatePullRequestOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreatePullRequestOptionBuilder {
        CreatePullRequestOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_create_pull_request() -> CreatePullRequestOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        CreatePullRequestOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<CreatePullRequestOption> for CreatePullRequestOptionBuilder {
    fn into(self) -> CreatePullRequestOption {
        self.body
    }
}

impl Into<CreatePullRequestOption> for CreatePullRequestOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    fn into(self) -> CreatePullRequestOption {
        self.inner.body
    }
}

/// Builder for [`CreatePullRequestOption`](./struct.CreatePullRequestOption.html) object.
#[derive(Debug, Clone)]
pub struct CreatePullRequestOptionBuilder {
    body: self::CreatePullRequestOption,
}

impl CreatePullRequestOptionBuilder {
    #[inline]
    pub fn assignee(mut self, value: impl Into<String>) -> Self {
        self.body.assignee = Some(value.into());
        self
    }

    #[inline]
    pub fn assignees(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.assignees = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.body.base = Some(value.into());
        self
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.body.due_date = Some(value.into());
        self
    }

    #[inline]
    pub fn head(mut self, value: impl Into<String>) -> Self {
        self.body.head = Some(value.into());
        self
    }

    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn milestone(mut self, value: impl Into<i64>) -> Self {
        self.body.milestone = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.body.title = Some(value.into());
        self
    }
}

/// Builder created by [`CreatePullRequestOption::repo_create_pull_request`](./struct.CreatePullRequestOption.html#method.repo_create_pull_request) method for a `POST` operation associated with `CreatePullRequestOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreatePullRequestOptionPostBuilder<Owner, Repo> {
    inner: CreatePullRequestOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct CreatePullRequestOptionPostBuilderContainer {
    body: self::CreatePullRequestOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> CreatePullRequestOptionPostBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreatePullRequestOptionPostBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreatePullRequestOptionPostBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn assignee(mut self, value: impl Into<String>) -> Self {
        self.inner.body.assignee = Some(value.into());
        self
    }

    #[inline]
    pub fn assignees(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.assignees = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.inner.body.base = Some(value.into());
        self
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.inner.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.inner.body.due_date = Some(value.into());
        self
    }

    #[inline]
    pub fn head(mut self, value: impl Into<String>) -> Self {
        self.inner.body.head = Some(value.into());
        self
    }

    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.inner.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn milestone(mut self, value: impl Into<i64>) -> Self {
        self.inner.body.milestone = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.inner.body.title = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreatePullRequestOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = crate::pull_request::PullRequest;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::pull_request::PullRequest, CreatePullRequestOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
