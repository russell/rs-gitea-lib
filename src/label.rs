
/// Label a label to an issue or a pr
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Label {
    pub color: Option<String>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub url: Option<String>,
}

impl Label {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> LabelBuilder {
        LabelBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn org_list_labels() -> LabelGetBuilder<crate::generics::MissingOrg> {
        LabelGetBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_get_label() -> LabelGetBuilder1<crate::generics::MissingOrg, crate::generics::MissingId> {
        LabelGetBuilder1 {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_get_labels() -> LabelGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        LabelGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_list_labels() -> LabelGetBuilder3<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        LabelGetBuilder3 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_get_label() -> LabelGetBuilder4<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        LabelGetBuilder4 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<Label> for LabelBuilder {
    fn into(self) -> Label {
        self.body
    }
}

/// Builder for [`Label`](./struct.Label.html) object.
#[derive(Debug, Clone)]
pub struct LabelBuilder {
    body: self::Label,
}

impl LabelBuilder {
    #[inline]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.body.color = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`Label::org_list_labels`](./struct.Label.html#method.org_list_labels) method for a `GET` operation associated with `Label`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct LabelGetBuilder<Org> {
    inner: LabelGetBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct LabelGetBuilderContainer {
    param_org: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Org> LabelGetBuilder<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> LabelGetBuilder<crate::generics::OrgExists> {
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for LabelGetBuilder<crate::generics::OrgExists> {
    type Output = Vec<Label>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/labels", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
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

/// Builder created by [`Label::org_get_label`](./struct.Label.html#method.org_get_label) method for a `GET` operation associated with `Label`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct LabelGetBuilder1<Org, Id> {
    inner: LabelGetBuilder1Container,
    _param_org: core::marker::PhantomData<Org>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct LabelGetBuilder1Container {
    param_org: Option<String>,
    param_id: Option<i64>,
}

impl<Org, Id> LabelGetBuilder1<Org, Id> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> LabelGetBuilder1<crate::generics::OrgExists, Id> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the label to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> LabelGetBuilder1<Org, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for LabelGetBuilder1<crate::generics::OrgExists, crate::generics::IdExists> {
    type Output = Label;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/labels/{id}", org=self.inner.param_org.as_ref().expect("missing parameter org?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`Label::issue_get_labels`](./struct.Label.html#method.issue_get_labels) method for a `GET` operation associated with `Label`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct LabelGetBuilder2<Owner, Repo, Index> {
    inner: LabelGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct LabelGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> LabelGetBuilder2<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> LabelGetBuilder2<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> LabelGetBuilder2<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> LabelGetBuilder2<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for LabelGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<Label>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/labels", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }
}

/// Builder created by [`Label::issue_list_labels`](./struct.Label.html#method.issue_list_labels) method for a `GET` operation associated with `Label`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct LabelGetBuilder3<Owner, Repo> {
    inner: LabelGetBuilder3Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct LabelGetBuilder3Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> LabelGetBuilder3<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> LabelGetBuilder3<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> LabelGetBuilder3<Owner, crate::generics::RepoExists> {
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for LabelGetBuilder3<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Label>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/labels", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
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

/// Builder created by [`Label::issue_get_label`](./struct.Label.html#method.issue_get_label) method for a `GET` operation associated with `Label`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct LabelGetBuilder4<Owner, Repo, Id> {
    inner: LabelGetBuilder4Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct LabelGetBuilder4Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> LabelGetBuilder4<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> LabelGetBuilder4<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> LabelGetBuilder4<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the label to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> LabelGetBuilder4<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for LabelGetBuilder4<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = Label;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/labels/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
