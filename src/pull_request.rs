
/// PullRequest represents a pull request
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub assignee: Option<crate::user::User>,
    pub assignees: Option<Vec<crate::user::User>>,
    pub base: Option<crate::pr_branch_info::PrBranchInfo>,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: Option<i64>,
    pub created_at: Option<String>,
    pub diff_url: Option<String>,
    pub due_date: Option<String>,
    pub head: Option<crate::pr_branch_info::PrBranchInfo>,
    pub html_url: Option<String>,
    pub id: Option<i64>,
    pub is_locked: Option<bool>,
    pub labels: Option<Vec<crate::label::Label>>,
    pub merge_base: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub mergeable: Option<bool>,
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    pub merged_by: Option<crate::user::User>,
    pub milestone: Option<crate::milestone::Milestone>,
    pub number: Option<i64>,
    pub patch_url: Option<String>,
    pub state: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub url: Option<String>,
    pub user: Option<crate::user::User>,
}

impl PullRequest {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PullRequestBuilder {
        PullRequestBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_pull_requests() -> PullRequestGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        PullRequestGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_pull_request() -> PullRequestGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        PullRequestGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<PullRequest> for PullRequestBuilder {
    fn into(self) -> PullRequest {
        self.body
    }
}

/// Builder for [`PullRequest`](./struct.PullRequest.html) object.
#[derive(Debug, Clone)]
pub struct PullRequestBuilder {
    body: self::PullRequest,
}

impl PullRequestBuilder {
    #[inline]
    pub fn assignee(mut self, value: crate::user::User) -> Self {
        self.body.assignee = Some(value.into());
        self
    }

    #[inline]
    pub fn assignees(mut self, value: impl Iterator<Item = crate::user::User>) -> Self {
        self.body.assignees = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn base(mut self, value: crate::pr_branch_info::PrBranchInfo) -> Self {
        self.body.base = Some(value.into());
        self
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn closed_at(mut self, value: impl Into<String>) -> Self {
        self.body.closed_at = Some(value.into());
        self
    }

    #[inline]
    pub fn comments(mut self, value: impl Into<i64>) -> Self {
        self.body.comments = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn diff_url(mut self, value: impl Into<String>) -> Self {
        self.body.diff_url = Some(value.into());
        self
    }

    #[inline]
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.body.due_date = Some(value.into());
        self
    }

    #[inline]
    pub fn head(mut self, value: crate::pr_branch_info::PrBranchInfo) -> Self {
        self.body.head = Some(value.into());
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
    pub fn is_locked(mut self, value: impl Into<bool>) -> Self {
        self.body.is_locked = Some(value.into());
        self
    }

    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = crate::label::Label>) -> Self {
        self.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn merge_base(mut self, value: impl Into<String>) -> Self {
        self.body.merge_base = Some(value.into());
        self
    }

    #[inline]
    pub fn merge_commit_sha(mut self, value: impl Into<String>) -> Self {
        self.body.merge_commit_sha = Some(value.into());
        self
    }

    #[inline]
    pub fn mergeable(mut self, value: impl Into<bool>) -> Self {
        self.body.mergeable = Some(value.into());
        self
    }

    #[inline]
    pub fn merged(mut self, value: impl Into<bool>) -> Self {
        self.body.merged = Some(value.into());
        self
    }

    #[inline]
    pub fn merged_at(mut self, value: impl Into<String>) -> Self {
        self.body.merged_at = Some(value.into());
        self
    }

    #[inline]
    pub fn merged_by(mut self, value: crate::user::User) -> Self {
        self.body.merged_by = Some(value.into());
        self
    }

    #[inline]
    pub fn milestone(mut self, value: crate::milestone::Milestone) -> Self {
        self.body.milestone = Some(value.into());
        self
    }

    #[inline]
    pub fn number(mut self, value: impl Into<i64>) -> Self {
        self.body.number = Some(value.into());
        self
    }

    #[inline]
    pub fn patch_url(mut self, value: impl Into<String>) -> Self {
        self.body.patch_url = Some(value.into());
        self
    }

    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.body.state = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.body.title = Some(value.into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }

    #[inline]
    pub fn user(mut self, value: crate::user::User) -> Self {
        self.body.user = Some(value.into());
        self
    }
}

/// Builder created by [`PullRequest::repo_list_pull_requests`](./struct.PullRequest.html#method.repo_list_pull_requests) method for a `GET` operation associated with `PullRequest`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PullRequestGetBuilder<Owner, Repo> {
    inner: PullRequestGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct PullRequestGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_state: Option<String>,
    param_sort: Option<String>,
    param_milestone: Option<i64>,
    param_labels: Option<crate::util::Delimited<i64, crate::util::Multi>>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> PullRequestGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PullRequestGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PullRequestGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// State of pull request: open or closed (optional)
    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.inner.param_state = Some(value.into());
        self
    }

    /// Type of sort
    #[inline]
    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.inner.param_sort = Some(value.into());
        self
    }

    /// ID of the milestone
    #[inline]
    pub fn milestone(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_milestone = Some(value.into());
        self
    }

    /// Label IDs
    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.inner.param_labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PullRequestGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<PullRequest>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("state", self.inner.param_state.as_ref().map(std::string::ToString::to_string)),
            ("sort", self.inner.param_sort.as_ref().map(std::string::ToString::to_string)),
            ("milestone", self.inner.param_milestone.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ])
        .query({
            &self.inner.param_labels.as_ref().map(|v| {
                v.iter().map(|v| ("labels", v.to_string())).collect::<Vec<_>>()
            }).unwrap_or_default()
        }))
    }
}

/// Builder created by [`PullRequest::repo_get_pull_request`](./struct.PullRequest.html#method.repo_get_pull_request) method for a `GET` operation associated with `PullRequest`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PullRequestGetBuilder1<Owner, Repo, Index> {
    inner: PullRequestGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct PullRequestGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> PullRequestGetBuilder1<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PullRequestGetBuilder1<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PullRequestGetBuilder1<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request to get
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> PullRequestGetBuilder1<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PullRequestGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = PullRequest;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }
}
