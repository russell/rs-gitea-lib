
/// Issue represents an issue in a repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PatchReposOwnerRepoIssuesIndexResponse {
    pub assignee: Option<crate::user::User>,
    pub assignees: Option<Vec<crate::user::User>>,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: Option<i64>,
    pub created_at: Option<String>,
    pub due_date: Option<String>,
    pub id: Option<i64>,
    pub labels: Option<Vec<crate::patch_repos_owner_repo_labels_id_response::PatchReposOwnerRepoLabelsIdResponse>>,
    pub milestone: Option<crate::patch_repos_owner_repo_milestones_id_response::PatchReposOwnerRepoMilestonesIdResponse>,
    pub number: Option<i64>,
    pub original_author: Option<String>,
    pub original_author_id: Option<i64>,
    pub pull_request: Option<crate::pull_request_meta::PullRequestMeta>,
    pub repository: Option<crate::repository_meta::RepositoryMeta>,
    pub state: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub url: Option<String>,
    pub user: Option<crate::user::User>,
}

impl PatchReposOwnerRepoIssuesIndexResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PatchReposOwnerRepoIssuesIndexResponseBuilder {
        PatchReposOwnerRepoIssuesIndexResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_search_issues() -> PatchReposOwnerRepoIssuesIndexResponseGetBuilder {
        PatchReposOwnerRepoIssuesIndexResponseGetBuilder {
            param_state: None,
            param_labels: None,
            param_page: None,
            param_q: None,
            param_priority_repo_id: None,
        }
    }

    #[inline]
    pub fn issue_list_issues() -> PatchReposOwnerRepoIssuesIndexResponseGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        PatchReposOwnerRepoIssuesIndexResponseGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_get_issue() -> PatchReposOwnerRepoIssuesIndexResponseGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        PatchReposOwnerRepoIssuesIndexResponseGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<PatchReposOwnerRepoIssuesIndexResponse> for PatchReposOwnerRepoIssuesIndexResponseBuilder {
    fn into(self) -> PatchReposOwnerRepoIssuesIndexResponse {
        self.body
    }
}

/// Builder for [`PatchReposOwnerRepoIssuesIndexResponse`](./struct.PatchReposOwnerRepoIssuesIndexResponse.html) object.
#[derive(Debug, Clone)]
pub struct PatchReposOwnerRepoIssuesIndexResponseBuilder {
    body: self::PatchReposOwnerRepoIssuesIndexResponse,
}

impl PatchReposOwnerRepoIssuesIndexResponseBuilder {
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
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.body.due_date = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = crate::patch_repos_owner_repo_labels_id_response::PatchReposOwnerRepoLabelsIdResponse>) -> Self {
        self.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn milestone(mut self, value: crate::patch_repos_owner_repo_milestones_id_response::PatchReposOwnerRepoMilestonesIdResponse) -> Self {
        self.body.milestone = Some(value.into());
        self
    }

    #[inline]
    pub fn number(mut self, value: impl Into<i64>) -> Self {
        self.body.number = Some(value.into());
        self
    }

    #[inline]
    pub fn original_author(mut self, value: impl Into<String>) -> Self {
        self.body.original_author = Some(value.into());
        self
    }

    #[inline]
    pub fn original_author_id(mut self, value: impl Into<i64>) -> Self {
        self.body.original_author_id = Some(value.into());
        self
    }

    #[inline]
    pub fn pull_request(mut self, value: crate::pull_request_meta::PullRequestMeta) -> Self {
        self.body.pull_request = Some(value.into());
        self
    }

    #[inline]
    pub fn repository(mut self, value: crate::repository_meta::RepositoryMeta) -> Self {
        self.body.repository = Some(value.into());
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

/// Builder created by [`PatchReposOwnerRepoIssuesIndexResponse::issue_search_issues`](./struct.PatchReposOwnerRepoIssuesIndexResponse.html#method.issue_search_issues) method for a `GET` operation associated with `PatchReposOwnerRepoIssuesIndexResponse`.
#[derive(Debug, Clone)]
pub struct PatchReposOwnerRepoIssuesIndexResponseGetBuilder {
    param_state: Option<String>,
    param_labels: Option<String>,
    param_page: Option<i64>,
    param_q: Option<String>,
    param_priority_repo_id: Option<i64>,
}

impl PatchReposOwnerRepoIssuesIndexResponseGetBuilder {
    /// whether issue is open or closed
    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.param_state = Some(value.into());
        self
    }

    /// comma separated list of labels. Fetch only issues that have any of this labels. Non existent labels are discarded
    #[inline]
    pub fn labels(mut self, value: impl Into<String>) -> Self {
        self.param_labels = Some(value.into());
        self
    }

    /// page number of requested issues
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// search string
    #[inline]
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.param_q = Some(value.into());
        self
    }

    /// repository to prioritize in the results
    #[inline]
    pub fn priority_repo_id(mut self, value: impl Into<i64>) -> Self {
        self.param_priority_repo_id = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PatchReposOwnerRepoIssuesIndexResponseGetBuilder {
    type Output = Vec<PatchReposOwnerRepoIssuesIndexResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/repos/issues/search".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("state", self.param_state.as_ref().map(std::string::ToString::to_string)),
            ("labels", self.param_labels.as_ref().map(std::string::ToString::to_string)),
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("q", self.param_q.as_ref().map(std::string::ToString::to_string)),
            ("priority_repo_id", self.param_priority_repo_id.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`PatchReposOwnerRepoIssuesIndexResponse::issue_list_issues`](./struct.PatchReposOwnerRepoIssuesIndexResponse.html#method.issue_list_issues) method for a `GET` operation associated with `PatchReposOwnerRepoIssuesIndexResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PatchReposOwnerRepoIssuesIndexResponseGetBuilder1<Owner, Repo> {
    inner: PatchReposOwnerRepoIssuesIndexResponseGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct PatchReposOwnerRepoIssuesIndexResponseGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_state: Option<String>,
    param_labels: Option<String>,
    param_page: Option<i64>,
    param_q: Option<String>,
}

impl<Owner, Repo> PatchReposOwnerRepoIssuesIndexResponseGetBuilder1<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PatchReposOwnerRepoIssuesIndexResponseGetBuilder1<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PatchReposOwnerRepoIssuesIndexResponseGetBuilder1<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// whether issue is open or closed
    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.inner.param_state = Some(value.into());
        self
    }

    /// comma separated list of labels. Fetch only issues that have any of this labels. Non existent labels are discarded
    #[inline]
    pub fn labels(mut self, value: impl Into<String>) -> Self {
        self.inner.param_labels = Some(value.into());
        self
    }

    /// page number of requested issues
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// search string
    #[inline]
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.inner.param_q = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PatchReposOwnerRepoIssuesIndexResponseGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<PatchReposOwnerRepoIssuesIndexResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("state", self.inner.param_state.as_ref().map(std::string::ToString::to_string)),
            ("labels", self.inner.param_labels.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("q", self.inner.param_q.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`PatchReposOwnerRepoIssuesIndexResponse::issue_get_issue`](./struct.PatchReposOwnerRepoIssuesIndexResponse.html#method.issue_get_issue) method for a `GET` operation associated with `PatchReposOwnerRepoIssuesIndexResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PatchReposOwnerRepoIssuesIndexResponseGetBuilder2<Owner, Repo, Index> {
    inner: PatchReposOwnerRepoIssuesIndexResponseGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct PatchReposOwnerRepoIssuesIndexResponseGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> PatchReposOwnerRepoIssuesIndexResponseGetBuilder2<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PatchReposOwnerRepoIssuesIndexResponseGetBuilder2<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PatchReposOwnerRepoIssuesIndexResponseGetBuilder2<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue to get
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> PatchReposOwnerRepoIssuesIndexResponseGetBuilder2<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PatchReposOwnerRepoIssuesIndexResponseGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = PatchReposOwnerRepoIssuesIndexResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }
}
