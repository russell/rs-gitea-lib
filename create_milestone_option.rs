
/// CreateMilestoneOption options for creating a milestone
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateMilestoneOption {
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub title: Option<String>,
}

impl CreateMilestoneOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateMilestoneOptionBuilder {
        CreateMilestoneOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_create_milestone() -> CreateMilestoneOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        CreateMilestoneOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<CreateMilestoneOption> for CreateMilestoneOptionBuilder {
    fn into(self) -> CreateMilestoneOption {
        self.body
    }
}

impl Into<CreateMilestoneOption> for CreateMilestoneOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    fn into(self) -> CreateMilestoneOption {
        self.inner.body
    }
}

/// Builder for [`CreateMilestoneOption`](./struct.CreateMilestoneOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateMilestoneOptionBuilder {
    body: self::CreateMilestoneOption,
}

impl CreateMilestoneOptionBuilder {
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
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.body.title = Some(value.into());
        self
    }
}

/// Builder created by [`CreateMilestoneOption::issue_create_milestone`](./struct.CreateMilestoneOption.html#method.issue_create_milestone) method for a `POST` operation associated with `CreateMilestoneOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateMilestoneOptionPostBuilder<Owner, Repo> {
    inner: CreateMilestoneOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct CreateMilestoneOptionPostBuilderContainer {
    body: self::CreateMilestoneOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> CreateMilestoneOptionPostBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateMilestoneOptionPostBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateMilestoneOptionPostBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
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
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.inner.body.title = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateMilestoneOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = crate::milestone::Milestone;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/milestones", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
