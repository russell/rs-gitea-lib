
/// PullReviewRequestOptions are options to add or remove pull review requests
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PullReviewRequestOptions {
    pub reviewers: Option<Vec<String>>,
    pub team_reviewers: Option<Vec<String>>,
}

impl PullReviewRequestOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PullReviewRequestOptionsBuilder {
        PullReviewRequestOptionsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_create_pull_review_requests() -> PullReviewRequestOptionsPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        PullReviewRequestOptionsPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_delete_pull_review_requests() -> PullReviewRequestOptionsDeleteBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        PullReviewRequestOptionsDeleteBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<PullReviewRequestOptions> for PullReviewRequestOptionsBuilder {
    fn into(self) -> PullReviewRequestOptions {
        self.body
    }
}

impl Into<PullReviewRequestOptions> for PullReviewRequestOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> PullReviewRequestOptions {
        self.inner.body
    }
}

impl Into<PullReviewRequestOptions> for PullReviewRequestOptionsDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> PullReviewRequestOptions {
        self.inner.body
    }
}

/// Builder for [`PullReviewRequestOptions`](./struct.PullReviewRequestOptions.html) object.
#[derive(Debug, Clone)]
pub struct PullReviewRequestOptionsBuilder {
    body: self::PullReviewRequestOptions,
}

impl PullReviewRequestOptionsBuilder {
    #[inline]
    pub fn reviewers(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.reviewers = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn team_reviewers(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.team_reviewers = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`PullReviewRequestOptions::repo_create_pull_review_requests`](./struct.PullReviewRequestOptions.html#method.repo_create_pull_review_requests) method for a `POST` operation associated with `PullReviewRequestOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PullReviewRequestOptionsPostBuilder<Owner, Repo, Index> {
    inner: PullReviewRequestOptionsPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct PullReviewRequestOptionsPostBuilderContainer {
    body: self::PullReviewRequestOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> PullReviewRequestOptionsPostBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PullReviewRequestOptionsPostBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PullReviewRequestOptionsPostBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> PullReviewRequestOptionsPostBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn reviewers(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.reviewers = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn team_reviewers(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.team_reviewers = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PullReviewRequestOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<crate::pull_review::PullReview>;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/requested_reviewers", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<Vec<crate::pull_review::PullReview>, PullReviewRequestOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`PullReviewRequestOptions::repo_delete_pull_review_requests`](./struct.PullReviewRequestOptions.html#method.repo_delete_pull_review_requests) method for a `DELETE` operation associated with `PullReviewRequestOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PullReviewRequestOptionsDeleteBuilder<Owner, Repo, Index> {
    inner: PullReviewRequestOptionsDeleteBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct PullReviewRequestOptionsDeleteBuilderContainer {
    body: self::PullReviewRequestOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> PullReviewRequestOptionsDeleteBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PullReviewRequestOptionsDeleteBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PullReviewRequestOptionsDeleteBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> PullReviewRequestOptionsDeleteBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn reviewers(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.reviewers = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn team_reviewers(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.team_reviewers = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PullReviewRequestOptionsDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/requested_reviewers", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}

impl crate::client::ResponseWrapper<serde_json::Value, PullReviewRequestOptionsDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
