
/// AddCollaboratorOption options when adding a user as a collaborator of a repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddCollaboratorOption {
    pub permission: Option<String>,
}

impl AddCollaboratorOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> AddCollaboratorOptionBuilder {
        AddCollaboratorOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_add_collaborator() -> AddCollaboratorOptionPutBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingCollaborator> {
        AddCollaboratorOptionPutBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_collaborator: core::marker::PhantomData,
        }
    }
}

impl Into<AddCollaboratorOption> for AddCollaboratorOptionBuilder {
    fn into(self) -> AddCollaboratorOption {
        self.body
    }
}

impl Into<AddCollaboratorOption> for AddCollaboratorOptionPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::CollaboratorExists> {
    fn into(self) -> AddCollaboratorOption {
        self.inner.body
    }
}

/// Builder for [`AddCollaboratorOption`](./struct.AddCollaboratorOption.html) object.
#[derive(Debug, Clone)]
pub struct AddCollaboratorOptionBuilder {
    body: self::AddCollaboratorOption,
}

impl AddCollaboratorOptionBuilder {
    #[inline]
    pub fn permission(mut self, value: impl Into<String>) -> Self {
        self.body.permission = Some(value.into());
        self
    }
}

/// Builder created by [`AddCollaboratorOption::repo_add_collaborator`](./struct.AddCollaboratorOption.html#method.repo_add_collaborator) method for a `PUT` operation associated with `AddCollaboratorOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AddCollaboratorOptionPutBuilder<Owner, Repo, Collaborator> {
    inner: AddCollaboratorOptionPutBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_collaborator: core::marker::PhantomData<Collaborator>,
}

#[derive(Debug, Default, Clone)]
struct AddCollaboratorOptionPutBuilderContainer {
    body: self::AddCollaboratorOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_collaborator: Option<String>,
}

impl<Owner, Repo, Collaborator> AddCollaboratorOptionPutBuilder<Owner, Repo, Collaborator> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> AddCollaboratorOptionPutBuilder<crate::generics::OwnerExists, Repo, Collaborator> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> AddCollaboratorOptionPutBuilder<Owner, crate::generics::RepoExists, Collaborator> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// username of the collaborator to add
    #[inline]
    pub fn collaborator(mut self, value: impl Into<String>) -> AddCollaboratorOptionPutBuilder<Owner, Repo, crate::generics::CollaboratorExists> {
        self.inner.param_collaborator = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn permission(mut self, value: impl Into<String>) -> Self {
        self.inner.body.permission = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for AddCollaboratorOptionPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::CollaboratorExists> {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::PUT;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/collaborators/{collaborator}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), collaborator=self.inner.param_collaborator.as_ref().expect("missing parameter collaborator?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}
