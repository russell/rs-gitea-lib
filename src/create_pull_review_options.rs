
/// CreatePullReviewOptions are options to create a pull review
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreatePullReviewOptions {
    pub body: Option<String>,
    pub comments: Option<Vec<crate::create_pull_review_comment::CreatePullReviewComment>>,
    pub commit_id: Option<String>,
    pub event: Option<String>,
}

impl CreatePullReviewOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreatePullReviewOptionsBuilder {
        CreatePullReviewOptionsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_create_pull_review() -> CreatePullReviewOptionsPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        CreatePullReviewOptionsPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<CreatePullReviewOptions> for CreatePullReviewOptionsBuilder {
    fn into(self) -> CreatePullReviewOptions {
        self.body
    }
}

impl Into<CreatePullReviewOptions> for CreatePullReviewOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> CreatePullReviewOptions {
        self.inner.body
    }
}

/// Builder for [`CreatePullReviewOptions`](./struct.CreatePullReviewOptions.html) object.
#[derive(Debug, Clone)]
pub struct CreatePullReviewOptionsBuilder {
    body: self::CreatePullReviewOptions,
}

impl CreatePullReviewOptionsBuilder {
    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn comments(mut self, value: impl Iterator<Item = crate::create_pull_review_comment::CreatePullReviewComment>) -> Self {
        self.body.comments = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn commit_id(mut self, value: impl Into<String>) -> Self {
        self.body.commit_id = Some(value.into());
        self
    }

    #[inline]
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.body.event = Some(value.into());
        self
    }
}

/// Builder created by [`CreatePullReviewOptions::repo_create_pull_review`](./struct.CreatePullReviewOptions.html#method.repo_create_pull_review) method for a `POST` operation associated with `CreatePullReviewOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreatePullReviewOptionsPostBuilder<Owner, Repo, Index> {
    inner: CreatePullReviewOptionsPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct CreatePullReviewOptionsPostBuilderContainer {
    body: self::CreatePullReviewOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> CreatePullReviewOptionsPostBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreatePullReviewOptionsPostBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreatePullReviewOptionsPostBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> CreatePullReviewOptionsPostBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.inner.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn comments(mut self, value: impl Iterator<Item = crate::create_pull_review_comment::CreatePullReviewComment>) -> Self {
        self.inner.body.comments = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn commit_id(mut self, value: impl Into<String>) -> Self {
        self.inner.body.commit_id = Some(value.into());
        self
    }

    #[inline]
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.inner.body.event = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreatePullReviewOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = crate::pull_review::PullReview;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/reviews", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::pull_review::PullReview, CreatePullReviewOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
