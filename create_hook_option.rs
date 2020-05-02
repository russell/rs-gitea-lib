
/// CreateHookOption options when create a hook
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateHookOption {
    pub active: Option<bool>,
    pub branch_filter: Option<String>,
    pub config: std::collections::BTreeMap<String, String>,
    pub events: Option<Vec<String>>,
}

impl CreateHookOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateHookOptionBuilder<crate::generics::MissingConfig> {
        CreateHookOptionBuilder {
            body: Default::default(),
            _config: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_create_hook() -> CreateHookOptionPostBuilder<crate::generics::MissingOrg, crate::generics::MissingConfig> {
        CreateHookOptionPostBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _config: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_create_hook() -> CreateHookOptionPostBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingConfig> {
        CreateHookOptionPostBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _config: core::marker::PhantomData,
        }
    }
}

impl Into<CreateHookOption> for CreateHookOptionBuilder<crate::generics::ConfigExists> {
    fn into(self) -> CreateHookOption {
        self.body
    }
}

impl Into<CreateHookOption> for CreateHookOptionPostBuilder<crate::generics::OrgExists, crate::generics::ConfigExists> {
    fn into(self) -> CreateHookOption {
        self.inner.body
    }
}

impl Into<CreateHookOption> for CreateHookOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ConfigExists> {
    fn into(self) -> CreateHookOption {
        self.inner.body
    }
}

/// Builder for [`CreateHookOption`](./struct.CreateHookOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateHookOptionBuilder<Config> {
    body: self::CreateHookOption,
    _config: core::marker::PhantomData<Config>,
}

impl<Config> CreateHookOptionBuilder<Config> {
    #[inline]
    pub fn active(mut self, value: impl Into<bool>) -> Self {
        self.body.active = Some(value.into());
        self
    }

    #[inline]
    pub fn branch_filter(mut self, value: impl Into<String>) -> Self {
        self.body.branch_filter = Some(value.into());
        self
    }

    #[inline]
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> CreateHookOptionBuilder<crate::generics::ConfigExists> {
        self.body.config = value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`CreateHookOption::org_create_hook`](./struct.CreateHookOption.html#method.org_create_hook) method for a `POST` operation associated with `CreateHookOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateHookOptionPostBuilder<Org, Config> {
    inner: CreateHookOptionPostBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
    _config: core::marker::PhantomData<Config>,
}

#[derive(Debug, Default, Clone)]
struct CreateHookOptionPostBuilderContainer {
    body: self::CreateHookOption,
    param_org: Option<String>,
}

impl<Org, Config> CreateHookOptionPostBuilder<Org, Config> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> CreateHookOptionPostBuilder<crate::generics::OrgExists, Config> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn active(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.active = Some(value.into());
        self
    }

    #[inline]
    pub fn branch_filter(mut self, value: impl Into<String>) -> Self {
        self.inner.body.branch_filter = Some(value.into());
        self
    }

    #[inline]
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> CreateHookOptionPostBuilder<Org, crate::generics::ConfigExists> {
        self.inner.body.config = value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateHookOptionPostBuilder<crate::generics::OrgExists, crate::generics::ConfigExists> {
    type Output = crate::post_repos_owner_repo_hooks_response::PostReposOwnerRepoHooksResponse;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/hooks/", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

/// Builder created by [`CreateHookOption::repo_create_hook`](./struct.CreateHookOption.html#method.repo_create_hook) method for a `POST` operation associated with `CreateHookOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateHookOptionPostBuilder1<Owner, Repo, Config> {
    inner: CreateHookOptionPostBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _config: core::marker::PhantomData<Config>,
}

#[derive(Debug, Default, Clone)]
struct CreateHookOptionPostBuilder1Container {
    body: self::CreateHookOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo, Config> CreateHookOptionPostBuilder1<Owner, Repo, Config> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateHookOptionPostBuilder1<crate::generics::OwnerExists, Repo, Config> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateHookOptionPostBuilder1<Owner, crate::generics::RepoExists, Config> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn active(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.active = Some(value.into());
        self
    }

    #[inline]
    pub fn branch_filter(mut self, value: impl Into<String>) -> Self {
        self.inner.body.branch_filter = Some(value.into());
        self
    }

    #[inline]
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> CreateHookOptionPostBuilder1<Owner, Repo, crate::generics::ConfigExists> {
        self.inner.body.config = value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateHookOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ConfigExists> {
    type Output = crate::post_repos_owner_repo_hooks_response::PostReposOwnerRepoHooksResponse;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
