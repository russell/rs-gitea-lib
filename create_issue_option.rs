
/// CreateIssueOption options to create one issue
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateIssueOption {
    /// username of assignee
    pub assignee: Option<String>,
    pub assignees: Option<Vec<String>>,
    pub body: Option<String>,
    pub closed: Option<bool>,
    pub due_date: Option<String>,
    /// list of label ids
    pub labels: Option<Vec<i64>>,
    /// milestone id
    pub milestone: Option<i64>,
    pub title: String,
}

impl CreateIssueOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateIssueOptionBuilder<crate::generics::MissingTitle> {
        CreateIssueOptionBuilder {
            body: Default::default(),
            _title: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_create_issue() -> CreateIssueOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingTitle> {
        CreateIssueOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _title: core::marker::PhantomData,
        }
    }
}

impl Into<CreateIssueOption> for CreateIssueOptionBuilder<crate::generics::TitleExists> {
    fn into(self) -> CreateIssueOption {
        self.body
    }
}

impl Into<CreateIssueOption> for CreateIssueOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::TitleExists> {
    fn into(self) -> CreateIssueOption {
        self.inner.body
    }
}

/// Builder for [`CreateIssueOption`](./struct.CreateIssueOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateIssueOptionBuilder<Title> {
    body: self::CreateIssueOption,
    _title: core::marker::PhantomData<Title>,
}

impl<Title> CreateIssueOptionBuilder<Title> {
    /// username of assignee
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
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn closed(mut self, value: impl Into<bool>) -> Self {
        self.body.closed = Some(value.into());
        self
    }

    #[inline]
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.body.due_date = Some(value.into());
        self
    }

    /// list of label ids
    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// milestone id
    #[inline]
    pub fn milestone(mut self, value: impl Into<i64>) -> Self {
        self.body.milestone = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> CreateIssueOptionBuilder<crate::generics::TitleExists> {
        self.body.title = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`CreateIssueOption::issue_create_issue`](./struct.CreateIssueOption.html#method.issue_create_issue) method for a `POST` operation associated with `CreateIssueOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateIssueOptionPostBuilder<Owner, Repo, Title> {
    inner: CreateIssueOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _title: core::marker::PhantomData<Title>,
}

#[derive(Debug, Default, Clone)]
struct CreateIssueOptionPostBuilderContainer {
    body: self::CreateIssueOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo, Title> CreateIssueOptionPostBuilder<Owner, Repo, Title> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateIssueOptionPostBuilder<crate::generics::OwnerExists, Repo, Title> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateIssueOptionPostBuilder<Owner, crate::generics::RepoExists, Title> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// username of assignee
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
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.inner.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn closed(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.closed = Some(value.into());
        self
    }

    #[inline]
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.inner.body.due_date = Some(value.into());
        self
    }

    /// list of label ids
    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.inner.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// milestone id
    #[inline]
    pub fn milestone(mut self, value: impl Into<i64>) -> Self {
        self.inner.body.milestone = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> CreateIssueOptionPostBuilder<Owner, Repo, crate::generics::TitleExists> {
        self.inner.body.title = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateIssueOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::TitleExists> {
    type Output = crate::issue::Issue;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
