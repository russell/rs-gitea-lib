
/// PullReviewComment represents a comment on a pull request review
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PullReviewComment {
    pub body: Option<String>,
    pub commit_id: Option<String>,
    pub created_at: Option<String>,
    pub diff_hunk: Option<String>,
    pub html_url: Option<String>,
    pub id: Option<i64>,
    pub original_commit_id: Option<String>,
    pub original_position: Option<i64>,
    pub path: Option<String>,
    pub position: Option<i64>,
    pub pull_request_review_id: Option<i64>,
    pub pull_request_url: Option<String>,
    pub updated_at: Option<String>,
    pub user: Option<crate::user::User>,
}

impl PullReviewComment {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PullReviewCommentBuilder {
        PullReviewCommentBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_get_pull_review_comments() -> PullReviewCommentGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingId> {
        PullReviewCommentGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<PullReviewComment> for PullReviewCommentBuilder {
    fn into(self) -> PullReviewComment {
        self.body
    }
}

/// Builder for [`PullReviewComment`](./struct.PullReviewComment.html) object.
#[derive(Debug, Clone)]
pub struct PullReviewCommentBuilder {
    body: self::PullReviewComment,
}

impl PullReviewCommentBuilder {
    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn commit_id(mut self, value: impl Into<String>) -> Self {
        self.body.commit_id = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn diff_hunk(mut self, value: impl Into<String>) -> Self {
        self.body.diff_hunk = Some(value.into());
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
    pub fn original_commit_id(mut self, value: impl Into<String>) -> Self {
        self.body.original_commit_id = Some(value.into());
        self
    }

    #[inline]
    pub fn original_position(mut self, value: impl Into<i64>) -> Self {
        self.body.original_position = Some(value.into());
        self
    }

    #[inline]
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.body.path = Some(value.into());
        self
    }

    #[inline]
    pub fn position(mut self, value: impl Into<i64>) -> Self {
        self.body.position = Some(value.into());
        self
    }

    #[inline]
    pub fn pull_request_review_id(mut self, value: impl Into<i64>) -> Self {
        self.body.pull_request_review_id = Some(value.into());
        self
    }

    #[inline]
    pub fn pull_request_url(mut self, value: impl Into<String>) -> Self {
        self.body.pull_request_url = Some(value.into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }

    #[inline]
    pub fn user(mut self, value: crate::user::User) -> Self {
        self.body.user = Some(value.into());
        self
    }
}

/// Builder created by [`PullReviewComment::repo_get_pull_review_comments`](./struct.PullReviewComment.html#method.repo_get_pull_review_comments) method for a `GET` operation associated with `PullReviewComment`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PullReviewCommentGetBuilder<Owner, Repo, Index, Id> {
    inner: PullReviewCommentGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct PullReviewCommentGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Index, Id> PullReviewCommentGetBuilder<Owner, Repo, Index, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PullReviewCommentGetBuilder<crate::generics::OwnerExists, Repo, Index, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PullReviewCommentGetBuilder<Owner, crate::generics::RepoExists, Index, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> PullReviewCommentGetBuilder<Owner, Repo, crate::generics::IndexExists, Id> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the review
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PullReviewCommentGetBuilder<Owner, Repo, Index, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PullReviewCommentGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists> {
    type Output = Vec<PullReviewComment>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/reviews/{id}/comments", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
