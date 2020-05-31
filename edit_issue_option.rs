
/// EditIssueOption options for editing an issue
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditIssueOption {
    pub assignee: Option<String>,
    pub assignees: Option<Vec<String>>,
    pub body: Option<String>,
    pub due_date: Option<String>,
    pub milestone: Option<i64>,
    pub state: Option<String>,
    pub title: Option<String>,
    pub unset_due_date: Option<bool>,
}

impl EditIssueOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditIssueOptionBuilder {
        EditIssueOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_edit_issue() -> EditIssueOptionPatchBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        EditIssueOptionPatchBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<EditIssueOption> for EditIssueOptionBuilder {
    fn into(self) -> EditIssueOption {
        self.body
    }
}

impl Into<EditIssueOption> for EditIssueOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> EditIssueOption {
        self.inner.body
    }
}

/// Builder for [`EditIssueOption`](./struct.EditIssueOption.html) object.
#[derive(Debug, Clone)]
pub struct EditIssueOptionBuilder {
    body: self::EditIssueOption,
}

impl EditIssueOptionBuilder {
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
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.body.due_date = Some(value.into());
        self
    }

    #[inline]
    pub fn milestone(mut self, value: impl Into<i64>) -> Self {
        self.body.milestone = Some(value.into());
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
    pub fn unset_due_date(mut self, value: impl Into<bool>) -> Self {
        self.body.unset_due_date = Some(value.into());
        self
    }
}

/// Builder created by [`EditIssueOption::issue_edit_issue`](./struct.EditIssueOption.html#method.issue_edit_issue) method for a `PATCH` operation associated with `EditIssueOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditIssueOptionPatchBuilder<Owner, Repo, Index> {
    inner: EditIssueOptionPatchBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct EditIssueOptionPatchBuilderContainer {
    body: self::EditIssueOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> EditIssueOptionPatchBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditIssueOptionPatchBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditIssueOptionPatchBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue to edit
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> EditIssueOptionPatchBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

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
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.inner.body.due_date = Some(value.into());
        self
    }

    #[inline]
    pub fn milestone(mut self, value: impl Into<i64>) -> Self {
        self.inner.body.milestone = Some(value.into());
        self
    }

    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.inner.body.state = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.inner.body.title = Some(value.into());
        self
    }

    #[inline]
    pub fn unset_due_date(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.unset_due_date = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditIssueOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = crate::issue::Issue;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::issue::Issue, EditIssueOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
