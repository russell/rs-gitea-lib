
/// Milestone milestone is a collection of issues on one repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PatchReposOwnerRepoMilestonesIdResponse {
    pub closed_at: Option<String>,
    pub closed_issues: Option<i64>,
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub id: Option<i64>,
    pub open_issues: Option<i64>,
    pub state: Option<String>,
    pub title: Option<String>,
}

impl PatchReposOwnerRepoMilestonesIdResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PatchReposOwnerRepoMilestonesIdResponseBuilder {
        PatchReposOwnerRepoMilestonesIdResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_get_milestones_list() -> PatchReposOwnerRepoMilestonesIdResponseGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        PatchReposOwnerRepoMilestonesIdResponseGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_get_milestone() -> PatchReposOwnerRepoMilestonesIdResponseGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        PatchReposOwnerRepoMilestonesIdResponseGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<PatchReposOwnerRepoMilestonesIdResponse> for PatchReposOwnerRepoMilestonesIdResponseBuilder {
    fn into(self) -> PatchReposOwnerRepoMilestonesIdResponse {
        self.body
    }
}

/// Builder for [`PatchReposOwnerRepoMilestonesIdResponse`](./struct.PatchReposOwnerRepoMilestonesIdResponse.html) object.
#[derive(Debug, Clone)]
pub struct PatchReposOwnerRepoMilestonesIdResponseBuilder {
    body: self::PatchReposOwnerRepoMilestonesIdResponse,
}

impl PatchReposOwnerRepoMilestonesIdResponseBuilder {
    #[inline]
    pub fn closed_at(mut self, value: impl Into<String>) -> Self {
        self.body.closed_at = Some(value.into());
        self
    }

    #[inline]
    pub fn closed_issues(mut self, value: impl Into<i64>) -> Self {
        self.body.closed_issues = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn due_on(mut self, value: impl Into<String>) -> Self {
        self.body.due_on = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn open_issues(mut self, value: impl Into<i64>) -> Self {
        self.body.open_issues = Some(value.into());
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
}

/// Builder created by [`PatchReposOwnerRepoMilestonesIdResponse::issue_get_milestones_list`](./struct.PatchReposOwnerRepoMilestonesIdResponse.html#method.issue_get_milestones_list) method for a `GET` operation associated with `PatchReposOwnerRepoMilestonesIdResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PatchReposOwnerRepoMilestonesIdResponseGetBuilder<Owner, Repo> {
    inner: PatchReposOwnerRepoMilestonesIdResponseGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct PatchReposOwnerRepoMilestonesIdResponseGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_state: Option<String>,
}

impl<Owner, Repo> PatchReposOwnerRepoMilestonesIdResponseGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PatchReposOwnerRepoMilestonesIdResponseGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PatchReposOwnerRepoMilestonesIdResponseGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// Milestone state, Recognised values are open, closed and all. Defaults to "open"
    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.inner.param_state = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PatchReposOwnerRepoMilestonesIdResponseGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<PatchReposOwnerRepoMilestonesIdResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/milestones", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("state", self.inner.param_state.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`PatchReposOwnerRepoMilestonesIdResponse::issue_get_milestone`](./struct.PatchReposOwnerRepoMilestonesIdResponse.html#method.issue_get_milestone) method for a `GET` operation associated with `PatchReposOwnerRepoMilestonesIdResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PatchReposOwnerRepoMilestonesIdResponseGetBuilder1<Owner, Repo, Id> {
    inner: PatchReposOwnerRepoMilestonesIdResponseGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct PatchReposOwnerRepoMilestonesIdResponseGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> PatchReposOwnerRepoMilestonesIdResponseGetBuilder1<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PatchReposOwnerRepoMilestonesIdResponseGetBuilder1<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PatchReposOwnerRepoMilestonesIdResponseGetBuilder1<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the milestone
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PatchReposOwnerRepoMilestonesIdResponseGetBuilder1<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PatchReposOwnerRepoMilestonesIdResponseGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = PatchReposOwnerRepoMilestonesIdResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/milestones/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
