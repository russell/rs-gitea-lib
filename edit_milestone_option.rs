
/// EditMilestoneOption options for editing a milestone
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditMilestoneOption {
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub state: Option<String>,
    pub title: Option<String>,
}

impl EditMilestoneOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditMilestoneOptionBuilder {
        EditMilestoneOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_edit_milestone() -> EditMilestoneOptionPatchBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        EditMilestoneOptionPatchBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<EditMilestoneOption> for EditMilestoneOptionBuilder {
    fn into(self) -> EditMilestoneOption {
        self.body
    }
}

impl Into<EditMilestoneOption> for EditMilestoneOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    fn into(self) -> EditMilestoneOption {
        self.inner.body
    }
}

/// Builder for [`EditMilestoneOption`](./struct.EditMilestoneOption.html) object.
#[derive(Debug, Clone)]
pub struct EditMilestoneOptionBuilder {
    body: self::EditMilestoneOption,
}

impl EditMilestoneOptionBuilder {
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

/// Builder created by [`EditMilestoneOption::issue_edit_milestone`](./struct.EditMilestoneOption.html#method.issue_edit_milestone) method for a `PATCH` operation associated with `EditMilestoneOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditMilestoneOptionPatchBuilder<Owner, Repo, Id> {
    inner: EditMilestoneOptionPatchBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditMilestoneOptionPatchBuilderContainer {
    body: self::EditMilestoneOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> EditMilestoneOptionPatchBuilder<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditMilestoneOptionPatchBuilder<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditMilestoneOptionPatchBuilder<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the milestone
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditMilestoneOptionPatchBuilder<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn due_on(mut self, value: impl Into<String>) -> Self {
        self.inner.body.due_on = Some(value.into());
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
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditMilestoneOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = crate::milestone::Milestone;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/milestones/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
