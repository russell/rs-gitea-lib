
/// Hook a hook is a web hook when one repository changed
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostReposOwnerRepoHooksResponse {
    pub active: Option<bool>,
    pub config: Option<std::collections::BTreeMap<String, String>>,
    pub created_at: Option<String>,
    pub events: Option<Vec<String>>,
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub updated_at: Option<String>,
}

impl PostReposOwnerRepoHooksResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PostReposOwnerRepoHooksResponseBuilder {
        PostReposOwnerRepoHooksResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn org_list_hooks() -> PostReposOwnerRepoHooksResponseGetBuilder<crate::generics::MissingOrg> {
        PostReposOwnerRepoHooksResponseGetBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_get_hook() -> PostReposOwnerRepoHooksResponseGetBuilder1<crate::generics::MissingOrg, crate::generics::MissingId> {
        PostReposOwnerRepoHooksResponseGetBuilder1 {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_list_hooks() -> PostReposOwnerRepoHooksResponseGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        PostReposOwnerRepoHooksResponseGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_hook() -> PostReposOwnerRepoHooksResponseGetBuilder3<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        PostReposOwnerRepoHooksResponseGetBuilder3 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<PostReposOwnerRepoHooksResponse> for PostReposOwnerRepoHooksResponseBuilder {
    fn into(self) -> PostReposOwnerRepoHooksResponse {
        self.body
    }
}

/// Builder for [`PostReposOwnerRepoHooksResponse`](./struct.PostReposOwnerRepoHooksResponse.html) object.
#[derive(Debug, Clone)]
pub struct PostReposOwnerRepoHooksResponseBuilder {
    body: self::PostReposOwnerRepoHooksResponse,
}

impl PostReposOwnerRepoHooksResponseBuilder {
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
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.body.type_ = Some(value.into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }
}

/// Builder created by [`PostReposOwnerRepoHooksResponse::org_list_hooks`](./struct.PostReposOwnerRepoHooksResponse.html#method.org_list_hooks) method for a `GET` operation associated with `PostReposOwnerRepoHooksResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PostReposOwnerRepoHooksResponseGetBuilder<Org> {
    inner: PostReposOwnerRepoHooksResponseGetBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct PostReposOwnerRepoHooksResponseGetBuilderContainer {
    param_org: Option<String>,
}

impl<Org> PostReposOwnerRepoHooksResponseGetBuilder<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> PostReposOwnerRepoHooksResponseGetBuilder<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PostReposOwnerRepoHooksResponseGetBuilder<crate::generics::OrgExists> {
    type Output = Vec<PostReposOwnerRepoHooksResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/hooks", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }
}

/// Builder created by [`PostReposOwnerRepoHooksResponse::org_get_hook`](./struct.PostReposOwnerRepoHooksResponse.html#method.org_get_hook) method for a `GET` operation associated with `PostReposOwnerRepoHooksResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PostReposOwnerRepoHooksResponseGetBuilder1<Org, Id> {
    inner: PostReposOwnerRepoHooksResponseGetBuilder1Container,
    _param_org: core::marker::PhantomData<Org>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct PostReposOwnerRepoHooksResponseGetBuilder1Container {
    param_org: Option<String>,
    param_id: Option<i64>,
}

impl<Org, Id> PostReposOwnerRepoHooksResponseGetBuilder1<Org, Id> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> PostReposOwnerRepoHooksResponseGetBuilder1<crate::generics::OrgExists, Id> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the hook to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PostReposOwnerRepoHooksResponseGetBuilder1<Org, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PostReposOwnerRepoHooksResponseGetBuilder1<crate::generics::OrgExists, crate::generics::IdExists> {
    type Output = PostReposOwnerRepoHooksResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/hooks/{id}", org=self.inner.param_org.as_ref().expect("missing parameter org?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`PostReposOwnerRepoHooksResponse::repo_list_hooks`](./struct.PostReposOwnerRepoHooksResponse.html#method.repo_list_hooks) method for a `GET` operation associated with `PostReposOwnerRepoHooksResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PostReposOwnerRepoHooksResponseGetBuilder2<Owner, Repo> {
    inner: PostReposOwnerRepoHooksResponseGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct PostReposOwnerRepoHooksResponseGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> PostReposOwnerRepoHooksResponseGetBuilder2<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PostReposOwnerRepoHooksResponseGetBuilder2<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PostReposOwnerRepoHooksResponseGetBuilder2<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PostReposOwnerRepoHooksResponseGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<PostReposOwnerRepoHooksResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`PostReposOwnerRepoHooksResponse::repo_get_hook`](./struct.PostReposOwnerRepoHooksResponse.html#method.repo_get_hook) method for a `GET` operation associated with `PostReposOwnerRepoHooksResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PostReposOwnerRepoHooksResponseGetBuilder3<Owner, Repo, Id> {
    inner: PostReposOwnerRepoHooksResponseGetBuilder3Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct PostReposOwnerRepoHooksResponseGetBuilder3Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> PostReposOwnerRepoHooksResponseGetBuilder3<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> PostReposOwnerRepoHooksResponseGetBuilder3<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> PostReposOwnerRepoHooksResponseGetBuilder3<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the hook to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PostReposOwnerRepoHooksResponseGetBuilder3<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PostReposOwnerRepoHooksResponseGetBuilder3<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = PostReposOwnerRepoHooksResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
