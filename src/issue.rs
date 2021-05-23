
/// Issue represents an issue in a repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub assignee: Option<crate::user::User>,
    pub assignees: Option<Vec<crate::user::User>>,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: Option<i64>,
    pub created_at: Option<String>,
    pub due_date: Option<String>,
    pub html_url: Option<String>,
    pub id: Option<i64>,
    pub is_locked: Option<bool>,
    pub labels: Option<Vec<crate::label::Label>>,
    pub milestone: Option<crate::milestone::Milestone>,
    pub number: Option<i64>,
    pub original_author: Option<String>,
    pub original_author_id: Option<i64>,
    pub pull_request: Option<crate::pull_request_meta::PullRequestMeta>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub repository: Option<crate::repository_meta::RepositoryMeta>,
    pub state: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub url: Option<String>,
    pub user: Option<crate::user::User>,
}

impl Issue {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> IssueBuilder {
        IssueBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_search_issues() -> IssueGetBuilder {
        IssueGetBuilder {
            param_state: None,
            param_labels: None,
            param_q: None,
            param_priority_repo_id: None,
            param_type: None,
            param_since: None,
            param_before: None,
            param_assigned: None,
            param_created: None,
            param_mentioned: None,
            param_review_requested: None,
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn issue_list_issues() -> IssueGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        IssueGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_get_issue() -> IssueGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        IssueGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<Issue> for IssueBuilder {
    fn into(self) -> Issue {
        self.body
    }
}

/// Builder for [`Issue`](./struct.Issue.html) object.
#[derive(Debug, Clone)]
pub struct IssueBuilder {
    body: self::Issue,
}

impl IssueBuilder {
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
    pub fn ref_(mut self, value: impl Into<String>) -> Self {
        self.body.ref_ = Some(value.into());
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

/// Builder created by [`Issue::issue_search_issues`](./struct.Issue.html#method.issue_search_issues) method for a `GET` operation associated with `Issue`.
#[derive(Debug, Clone)]
pub struct IssueGetBuilder {
    param_state: Option<String>,
    param_labels: Option<String>,
    param_q: Option<String>,
    param_priority_repo_id: Option<i64>,
    param_type: Option<String>,
    param_since: Option<String>,
    param_before: Option<String>,
    param_assigned: Option<bool>,
    param_created: Option<bool>,
    param_mentioned: Option<bool>,
    param_review_requested: Option<bool>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl IssueGetBuilder {
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

    /// filter by type (issues / pulls) if set
    #[inline]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.param_type = Some(value.into());
        self
    }

    /// Only show notifications updated after the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.param_since = Some(value.into());
        self
    }

    /// Only show notifications updated before the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.param_before = Some(value.into());
        self
    }

    /// filter (issues / pulls) assigned to you, default is false
    #[inline]
    pub fn assigned(mut self, value: impl Into<bool>) -> Self {
        self.param_assigned = Some(value.into());
        self
    }

    /// filter (issues / pulls) created by you, default is false
    #[inline]
    pub fn created(mut self, value: impl Into<bool>) -> Self {
        self.param_created = Some(value.into());
        self
    }

    /// filter (issues / pulls) mentioning you, default is false
    #[inline]
    pub fn mentioned(mut self, value: impl Into<bool>) -> Self {
        self.param_mentioned = Some(value.into());
        self
    }

    /// filter pulls requesting your review, default is false
    #[inline]
    pub fn review_requested(mut self, value: impl Into<bool>) -> Self {
        self.param_review_requested = Some(value.into());
        self
    }

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
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for IssueGetBuilder {
    type Output = Vec<Issue>;

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
            ("q", self.param_q.as_ref().map(std::string::ToString::to_string)),
            ("priority_repo_id", self.param_priority_repo_id.as_ref().map(std::string::ToString::to_string)),
            ("type", self.param_type.as_ref().map(std::string::ToString::to_string)),
            ("since", self.param_since.as_ref().map(std::string::ToString::to_string)),
            ("before", self.param_before.as_ref().map(std::string::ToString::to_string)),
            ("assigned", self.param_assigned.as_ref().map(std::string::ToString::to_string)),
            ("created", self.param_created.as_ref().map(std::string::ToString::to_string)),
            ("mentioned", self.param_mentioned.as_ref().map(std::string::ToString::to_string)),
            ("review_requested", self.param_review_requested.as_ref().map(std::string::ToString::to_string)),
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Issue::issue_list_issues`](./struct.Issue.html#method.issue_list_issues) method for a `GET` operation associated with `Issue`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct IssueGetBuilder1<Owner, Repo> {
    inner: IssueGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct IssueGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_state: Option<String>,
    param_labels: Option<String>,
    param_q: Option<String>,
    param_type: Option<String>,
    param_milestones: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> IssueGetBuilder1<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> IssueGetBuilder1<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> IssueGetBuilder1<Owner, crate::generics::RepoExists> {
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

    /// search string
    #[inline]
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.inner.param_q = Some(value.into());
        self
    }

    /// filter by type (issues / pulls) if set
    #[inline]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.inner.param_type = Some(value.into());
        self
    }

    /// comma separated list of milestone names or ids. It uses names and fall back to ids. Fetch only issues that have any of this milestones. Non existent milestones are discarded
    #[inline]
    pub fn milestones(mut self, value: impl Into<String>) -> Self {
        self.inner.param_milestones = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for IssueGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Issue>;

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
            ("q", self.inner.param_q.as_ref().map(std::string::ToString::to_string)),
            ("type", self.inner.param_type.as_ref().map(std::string::ToString::to_string)),
            ("milestones", self.inner.param_milestones.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Issue::issue_get_issue`](./struct.Issue.html#method.issue_get_issue) method for a `GET` operation associated with `Issue`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct IssueGetBuilder2<Owner, Repo, Index> {
    inner: IssueGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct IssueGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> IssueGetBuilder2<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> IssueGetBuilder2<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> IssueGetBuilder2<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue to get
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> IssueGetBuilder2<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for IssueGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Issue;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }
}
