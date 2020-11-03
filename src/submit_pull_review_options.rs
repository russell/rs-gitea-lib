
/// SubmitPullReviewOptions are options to submit a pending pull review
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubmitPullReviewOptions {
    pub body: Option<String>,
    pub event: Option<String>,
}

impl SubmitPullReviewOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> SubmitPullReviewOptionsBuilder {
        SubmitPullReviewOptionsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_submit_pull_review() -> SubmitPullReviewOptionsPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingId> {
        SubmitPullReviewOptionsPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<SubmitPullReviewOptions> for SubmitPullReviewOptionsBuilder {
    fn into(self) -> SubmitPullReviewOptions {
        self.body
    }
}

impl Into<SubmitPullReviewOptions> for SubmitPullReviewOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists> {
    fn into(self) -> SubmitPullReviewOptions {
        self.inner.body
    }
}

/// Builder for [`SubmitPullReviewOptions`](./struct.SubmitPullReviewOptions.html) object.
#[derive(Debug, Clone)]
pub struct SubmitPullReviewOptionsBuilder {
    body: self::SubmitPullReviewOptions,
}

impl SubmitPullReviewOptionsBuilder {
    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.body.event = Some(value.into());
        self
    }
}

/// Builder created by [`SubmitPullReviewOptions::repo_submit_pull_review`](./struct.SubmitPullReviewOptions.html#method.repo_submit_pull_review) method for a `POST` operation associated with `SubmitPullReviewOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SubmitPullReviewOptionsPostBuilder<Owner, Repo, Index, Id> {
    inner: SubmitPullReviewOptionsPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct SubmitPullReviewOptionsPostBuilderContainer {
    body: self::SubmitPullReviewOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Index, Id> SubmitPullReviewOptionsPostBuilder<Owner, Repo, Index, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> SubmitPullReviewOptionsPostBuilder<crate::generics::OwnerExists, Repo, Index, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> SubmitPullReviewOptionsPostBuilder<Owner, crate::generics::RepoExists, Index, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> SubmitPullReviewOptionsPostBuilder<Owner, Repo, crate::generics::IndexExists, Id> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the review
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> SubmitPullReviewOptionsPostBuilder<Owner, Repo, Index, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.inner.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.inner.body.event = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for SubmitPullReviewOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists> {
    type Output = crate::pull_review::PullReview;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/reviews/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::pull_review::PullReview, SubmitPullReviewOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
