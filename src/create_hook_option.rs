
/// CreateHookOption options when create a hook
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateHookOption {
    pub active: Option<bool>,
    pub branch_filter: Option<String>,
    pub config: std::collections::BTreeMap<String, String>,
    pub events: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: crate::create_hook_option::CreateHookOptionType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CreateHookOptionType {
    #[serde(rename = "dingtalk")]
    Dingtalk,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "gitea")]
    Gitea,
    #[serde(rename = "gogs")]
    Gogs,
    #[serde(rename = "msteams")]
    Msteams,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "feishu")]
    Feishu,
}
impl Default for CreateHookOptionType {
    fn default() -> Self {
        CreateHookOptionType::Dingtalk
    }
}

impl CreateHookOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateHookOptionBuilder<crate::generics::MissingConfig, crate::generics::MissingType> {
        CreateHookOptionBuilder {
            body: Default::default(),
            _config: core::marker::PhantomData,
            _type: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_create_hook() -> CreateHookOptionPostBuilder<crate::generics::MissingOrg, crate::generics::MissingConfig, crate::generics::MissingType> {
        CreateHookOptionPostBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _config: core::marker::PhantomData,
            _type: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_create_hook() -> CreateHookOptionPostBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingConfig, crate::generics::MissingType> {
        CreateHookOptionPostBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _config: core::marker::PhantomData,
            _type: core::marker::PhantomData,
        }
    }
}

impl Into<CreateHookOption> for CreateHookOptionBuilder<crate::generics::ConfigExists, crate::generics::TypeExists> {
    fn into(self) -> CreateHookOption {
        self.body
    }
}

impl Into<CreateHookOption> for CreateHookOptionPostBuilder<crate::generics::OrgExists, crate::generics::ConfigExists, crate::generics::TypeExists> {
    fn into(self) -> CreateHookOption {
        self.inner.body
    }
}

impl Into<CreateHookOption> for CreateHookOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ConfigExists, crate::generics::TypeExists> {
    fn into(self) -> CreateHookOption {
        self.inner.body
    }
}

/// Builder for [`CreateHookOption`](./struct.CreateHookOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateHookOptionBuilder<Config, Type> {
    body: self::CreateHookOption,
    _config: core::marker::PhantomData<Config>,
    _type: core::marker::PhantomData<Type>,
}

impl<Config, Type> CreateHookOptionBuilder<Config, Type> {
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
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> CreateHookOptionBuilder<crate::generics::ConfigExists, Type> {
        self.body.config = value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn type_(mut self, value: crate::create_hook_option::CreateHookOptionType) -> CreateHookOptionBuilder<Config, crate::generics::TypeExists> {
        self.body.type_ = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`CreateHookOption::org_create_hook`](./struct.CreateHookOption.html#method.org_create_hook) method for a `POST` operation associated with `CreateHookOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateHookOptionPostBuilder<Org, Config, Type> {
    inner: CreateHookOptionPostBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
    _config: core::marker::PhantomData<Config>,
    _type: core::marker::PhantomData<Type>,
}

#[derive(Debug, Default, Clone)]
struct CreateHookOptionPostBuilderContainer {
    body: self::CreateHookOption,
    param_org: Option<String>,
}

impl<Org, Config, Type> CreateHookOptionPostBuilder<Org, Config, Type> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> CreateHookOptionPostBuilder<crate::generics::OrgExists, Config, Type> {
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
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> CreateHookOptionPostBuilder<Org, crate::generics::ConfigExists, Type> {
        self.inner.body.config = value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn type_(mut self, value: crate::create_hook_option::CreateHookOptionType) -> CreateHookOptionPostBuilder<Org, Config, crate::generics::TypeExists> {
        self.inner.body.type_ = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateHookOptionPostBuilder<crate::generics::OrgExists, crate::generics::ConfigExists, crate::generics::TypeExists> {
    type Output = crate::hook::Hook;

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
pub struct CreateHookOptionPostBuilder1<Owner, Repo, Config, Type> {
    inner: CreateHookOptionPostBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _config: core::marker::PhantomData<Config>,
    _type: core::marker::PhantomData<Type>,
}

#[derive(Debug, Default, Clone)]
struct CreateHookOptionPostBuilder1Container {
    body: self::CreateHookOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo, Config, Type> CreateHookOptionPostBuilder1<Owner, Repo, Config, Type> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateHookOptionPostBuilder1<crate::generics::OwnerExists, Repo, Config, Type> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateHookOptionPostBuilder1<Owner, crate::generics::RepoExists, Config, Type> {
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
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> CreateHookOptionPostBuilder1<Owner, Repo, crate::generics::ConfigExists, Type> {
        self.inner.body.config = value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn type_(mut self, value: crate::create_hook_option::CreateHookOptionType) -> CreateHookOptionPostBuilder1<Owner, Repo, Config, crate::generics::TypeExists> {
        self.inner.body.type_ = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateHookOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ConfigExists, crate::generics::TypeExists> {
    type Output = crate::hook::Hook;

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

