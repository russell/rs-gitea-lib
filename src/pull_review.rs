
/// PullReview represents a pull request review
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PullReview {
    pub body: Option<String>,
    pub comments_count: Option<i64>,
    pub commit_id: Option<String>,
    pub dismissed: Option<bool>,
    pub html_url: Option<String>,
    pub id: Option<i64>,
    pub official: Option<bool>,
    pub pull_request_url: Option<String>,
    pub stale: Option<bool>,
    pub state: Option<String>,
    pub submitted_at: Option<String>,
    pub team: Option<crate::team::Team>,
    pub user: Option<crate::user::User>,
}

impl PullReview {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PullReviewBuilder {
        PullReviewBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_pull_reviews() -> PullReviewGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        PullReviewGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_pull_review() -> PullReviewGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingId> {
        PullReviewGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_un_dismiss_pull_review() -> PullReviewPostBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingId> {
        PullReviewPostBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<PullReview> for PullReviewBuilder {
    fn into(self) -> PullReview {
        self.body
    }
}

/// Builder for [`PullReview`](./struct.PullReview.html) object.
#[derive(Debug, Clone)]
pub struct PullReviewBuilder {
    body: self::PullReview,
}

impl PullReviewBuilder {
    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn comments_count(mut self, value: impl Into<i64>) -> Self {
        self.body.comments_count = Some(value.into());
        self
    }

    #[inline]
    pub fn commit_id(mut self, value: impl Into<String>) -> Self {
        self.body.commit_id = Some(value.into());
        self
    }

    #[inline]
    pub fn dismissed(mut self, value: impl Into<bool>) -> Self {
        self.body.dismissed = Some(value.into());
        self
    }

    #[inline]
    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.body.html_url = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn official(mut self, value: impl Into<bool>) -> Self {
        self.body.official = Some(value.into());
        self
    }

    #[inline]
    pub fn pull_request_url(mut self, value: impl Into<String>) -> Self {
        self.body.pull_request_url = Some(value.into());
        self
    }

    #[inline]
    pub fn stale(mut self, value: impl Into<bool>) -> Self {
        self.body.stale = Some(value.into());
        self
    }

    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.body.state = Some(value.into());
        self
    }

    #[inline]
    pub fn submitted_at(mut self, value: impl Into<String>) -> Self {
        self.body.submitted_at = Some(value.into());
        self
    }

    #[inline]
    pub fn team(mut self, value: crate::team::Team) -> Self {
        self.body.team = Some(value.into());
        self
    }

    #[inline]
    pub fn user(mut self, value: crate::user::User) -> Self {
        self.body.user = Some(value.into());
        self
    }
}

/// Builder created by [`PullReview::repo_list_pull_reviews`](./struct.PullReview.html#method.repo_list_pull_reviews) method for a `GET` operation associated with `PullReview`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PullReviewGetBuilder<Owner, Repo, Index> {
    inner: PullReviewGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct PullReviewGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo, Index> PullReviewGetBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PullReviewGetBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PullReviewGetBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> PullReviewGetBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PullReviewGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<PullReview>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/reviews", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`PullReview::repo_get_pull_review`](./struct.PullReview.html#method.repo_get_pull_review) method for a `GET` operation associated with `PullReview`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PullReviewGetBuilder1<Owner, Repo, Index, Id> {
    inner: PullReviewGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct PullReviewGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Index, Id> PullReviewGetBuilder1<Owner, Repo, Index, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PullReviewGetBuilder1<crate::generics::OwnerExists, Repo, Index, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PullReviewGetBuilder1<Owner, crate::generics::RepoExists, Index, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> PullReviewGetBuilder1<Owner, Repo, crate::generics::IndexExists, Id> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the review
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PullReviewGetBuilder1<Owner, Repo, Index, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PullReviewGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists> {
    type Output = PullReview;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/reviews/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`PullReview::repo_un_dismiss_pull_review`](./struct.PullReview.html#method.repo_un_dismiss_pull_review) method for a `POST` operation associated with `PullReview`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PullReviewPostBuilder2<Owner, Repo, Index, Id> {
    inner: PullReviewPostBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct PullReviewPostBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Index, Id> PullReviewPostBuilder2<Owner, Repo, Index, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PullReviewPostBuilder2<crate::generics::OwnerExists, Repo, Index, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PullReviewPostBuilder2<Owner, crate::generics::RepoExists, Index, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> PullReviewPostBuilder2<Owner, Repo, crate::generics::IndexExists, Id> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the review
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PullReviewPostBuilder2<Owner, Repo, Index, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PullReviewPostBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists> {
    type Output = PullReview;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/reviews/{id}/undismissals", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

impl crate::client::ResponseWrapper<PullReview, PullReviewPostBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
