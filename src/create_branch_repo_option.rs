
/// CreateBranchRepoOption options when creating a branch in a repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateBranchRepoOption {
    /// Name of the branch to create
    pub new_branch_name: String,
    /// Name of the old branch to create from
    pub old_branch_name: Option<String>,
}

impl CreateBranchRepoOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateBranchRepoOptionBuilder<crate::generics::MissingNewBranchName> {
        CreateBranchRepoOptionBuilder {
            body: Default::default(),
            _new_branch_name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_create_branch() -> CreateBranchRepoOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingNewBranchName> {
        CreateBranchRepoOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _new_branch_name: core::marker::PhantomData,
        }
    }
}

impl Into<CreateBranchRepoOption> for CreateBranchRepoOptionBuilder<crate::generics::NewBranchNameExists> {
    fn into(self) -> CreateBranchRepoOption {
        self.body
    }
}

impl Into<CreateBranchRepoOption> for CreateBranchRepoOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NewBranchNameExists> {
    fn into(self) -> CreateBranchRepoOption {
        self.inner.body
    }
}

/// Builder for [`CreateBranchRepoOption`](./struct.CreateBranchRepoOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateBranchRepoOptionBuilder<NewBranchName> {
    body: self::CreateBranchRepoOption,
    _new_branch_name: core::marker::PhantomData<NewBranchName>,
}

impl<NewBranchName> CreateBranchRepoOptionBuilder<NewBranchName> {
    /// Name of the branch to create
    #[inline]
    pub fn new_branch_name(mut self, value: impl Into<String>) -> CreateBranchRepoOptionBuilder<crate::generics::NewBranchNameExists> {
        self.body.new_branch_name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Name of the old branch to create from
    #[inline]
    pub fn old_branch_name(mut self, value: impl Into<String>) -> Self {
        self.body.old_branch_name = Some(value.into());
        self
    }
}

/// Builder created by [`CreateBranchRepoOption::repo_create_branch`](./struct.CreateBranchRepoOption.html#method.repo_create_branch) method for a `POST` operation associated with `CreateBranchRepoOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateBranchRepoOptionPostBuilder<Owner, Repo, NewBranchName> {
    inner: CreateBranchRepoOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _new_branch_name: core::marker::PhantomData<NewBranchName>,
}

#[derive(Debug, Default, Clone)]
struct CreateBranchRepoOptionPostBuilderContainer {
    body: self::CreateBranchRepoOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo, NewBranchName> CreateBranchRepoOptionPostBuilder<Owner, Repo, NewBranchName> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateBranchRepoOptionPostBuilder<crate::generics::OwnerExists, Repo, NewBranchName> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateBranchRepoOptionPostBuilder<Owner, crate::generics::RepoExists, NewBranchName> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// Name of the branch to create
    #[inline]
    pub fn new_branch_name(mut self, value: impl Into<String>) -> CreateBranchRepoOptionPostBuilder<Owner, Repo, crate::generics::NewBranchNameExists> {
        self.inner.body.new_branch_name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Name of the old branch to create from
    #[inline]
    pub fn old_branch_name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.old_branch_name = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateBranchRepoOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NewBranchNameExists> {
    type Output = crate::branch::Branch;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/branches", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
