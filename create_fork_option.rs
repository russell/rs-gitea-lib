
/// CreateForkOption options for creating a fork
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateForkOption {
    /// organization name, if forking into an organization
    pub organization: Option<String>,
}

impl CreateForkOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateForkOptionBuilder {
        CreateForkOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn create_fork() -> CreateForkOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        CreateForkOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<CreateForkOption> for CreateForkOptionBuilder {
    fn into(self) -> CreateForkOption {
        self.body
    }
}

impl Into<CreateForkOption> for CreateForkOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    fn into(self) -> CreateForkOption {
        self.inner.body
    }
}

/// Builder for [`CreateForkOption`](./struct.CreateForkOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateForkOptionBuilder {
    body: self::CreateForkOption,
}

impl CreateForkOptionBuilder {
    /// organization name, if forking into an organization
    #[inline]
    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.body.organization = Some(value.into());
        self
    }
}

/// Builder created by [`CreateForkOption::create_fork`](./struct.CreateForkOption.html#method.create_fork) method for a `POST` operation associated with `CreateForkOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateForkOptionPostBuilder<Owner, Repo> {
    inner: CreateForkOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct CreateForkOptionPostBuilderContainer {
    body: self::CreateForkOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> CreateForkOptionPostBuilder<Owner, Repo> {
    /// owner of the repo to fork
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateForkOptionPostBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo to fork
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateForkOptionPostBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// organization name, if forking into an organization
    #[inline]
    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.inner.body.organization = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateForkOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = crate::repository::Repository;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/forks", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
