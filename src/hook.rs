
/// Hook a hook is a web hook when one repository changed
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Hook {
    pub active: Option<bool>,
    pub config: Option<std::collections::BTreeMap<String, String>>,
    pub created_at: Option<String>,
    pub events: Option<Vec<String>>,
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub updated_at: Option<String>,
}

impl Hook {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> HookBuilder {
        HookBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn org_list_hooks() -> HookGetBuilder<crate::generics::MissingOrg> {
        HookGetBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_get_hook() -> HookGetBuilder1<crate::generics::MissingOrg, crate::generics::MissingId> {
        HookGetBuilder1 {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_list_hooks() -> HookGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        HookGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_hook() -> HookGetBuilder3<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        HookGetBuilder3 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<Hook> for HookBuilder {
    fn into(self) -> Hook {
        self.body
    }
}

/// Builder for [`Hook`](./struct.Hook.html) object.
#[derive(Debug, Clone)]
pub struct HookBuilder {
    body: self::Hook,
}

impl HookBuilder {
    #[inline]
    pub fn active(mut self, value: impl Into<bool>) -> Self {
        self.body.active = Some(value.into());
        self
    }

    #[inline]
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> Self {
        self.body.config = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.body.type_ = Some(value.into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }
}

/// Builder created by [`Hook::org_list_hooks`](./struct.Hook.html#method.org_list_hooks) method for a `GET` operation associated with `Hook`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct HookGetBuilder<Org> {
    inner: HookGetBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct HookGetBuilderContainer {
    param_org: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Org> HookGetBuilder<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> HookGetBuilder<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for HookGetBuilder<crate::generics::OrgExists> {
    type Output = Vec<Hook>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/hooks", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Hook::org_get_hook`](./struct.Hook.html#method.org_get_hook) method for a `GET` operation associated with `Hook`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct HookGetBuilder1<Org, Id> {
    inner: HookGetBuilder1Container,
    _param_org: core::marker::PhantomData<Org>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct HookGetBuilder1Container {
    param_org: Option<String>,
    param_id: Option<i64>,
}

impl<Org, Id> HookGetBuilder1<Org, Id> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> HookGetBuilder1<crate::generics::OrgExists, Id> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the hook to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> HookGetBuilder1<Org, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for HookGetBuilder1<crate::generics::OrgExists, crate::generics::IdExists> {
    type Output = Hook;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/hooks/{id}", org=self.inner.param_org.as_ref().expect("missing parameter org?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`Hook::repo_list_hooks`](./struct.Hook.html#method.repo_list_hooks) method for a `GET` operation associated with `Hook`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct HookGetBuilder2<Owner, Repo> {
    inner: HookGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct HookGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> HookGetBuilder2<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> HookGetBuilder2<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> HookGetBuilder2<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for HookGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Hook>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Hook::repo_get_hook`](./struct.Hook.html#method.repo_get_hook) method for a `GET` operation associated with `Hook`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct HookGetBuilder3<Owner, Repo, Id> {
    inner: HookGetBuilder3Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct HookGetBuilder3Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> HookGetBuilder3<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> HookGetBuilder3<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> HookGetBuilder3<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the hook to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> HookGetBuilder3<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for HookGetBuilder3<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = Hook;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
