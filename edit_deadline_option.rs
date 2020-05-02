
/// EditDeadlineOption options for creating a deadline
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditDeadlineOption {
    pub due_date: String,
}

impl EditDeadlineOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditDeadlineOptionBuilder<crate::generics::MissingDueDate> {
        EditDeadlineOptionBuilder {
            body: Default::default(),
            _due_date: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_edit_issue_deadline() -> EditDeadlineOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingDueDate> {
        EditDeadlineOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _due_date: core::marker::PhantomData,
        }
    }
}

impl Into<EditDeadlineOption> for EditDeadlineOptionBuilder<crate::generics::DueDateExists> {
    fn into(self) -> EditDeadlineOption {
        self.body
    }
}

impl Into<EditDeadlineOption> for EditDeadlineOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::DueDateExists> {
    fn into(self) -> EditDeadlineOption {
        self.inner.body
    }
}

/// Builder for [`EditDeadlineOption`](./struct.EditDeadlineOption.html) object.
#[derive(Debug, Clone)]
pub struct EditDeadlineOptionBuilder<DueDate> {
    body: self::EditDeadlineOption,
    _due_date: core::marker::PhantomData<DueDate>,
}

impl<DueDate> EditDeadlineOptionBuilder<DueDate> {
    #[inline]
    pub fn due_date(mut self, value: impl Into<String>) -> EditDeadlineOptionBuilder<crate::generics::DueDateExists> {
        self.body.due_date = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`EditDeadlineOption::issue_edit_issue_deadline`](./struct.EditDeadlineOption.html#method.issue_edit_issue_deadline) method for a `POST` operation associated with `EditDeadlineOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditDeadlineOptionPostBuilder<Owner, Repo, Index, DueDate> {
    inner: EditDeadlineOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _due_date: core::marker::PhantomData<DueDate>,
}

#[derive(Debug, Default, Clone)]
struct EditDeadlineOptionPostBuilderContainer {
    body: self::EditDeadlineOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index, DueDate> EditDeadlineOptionPostBuilder<Owner, Repo, Index, DueDate> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditDeadlineOptionPostBuilder<crate::generics::OwnerExists, Repo, Index, DueDate> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditDeadlineOptionPostBuilder<Owner, crate::generics::RepoExists, Index, DueDate> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue to create or update a deadline on
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> EditDeadlineOptionPostBuilder<Owner, Repo, crate::generics::IndexExists, DueDate> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn due_date(mut self, value: impl Into<String>) -> EditDeadlineOptionPostBuilder<Owner, Repo, Index, crate::generics::DueDateExists> {
        self.inner.body.due_date = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditDeadlineOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::DueDateExists> {
    type Output = crate::issue_deadline::IssueDeadline;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/deadline", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
