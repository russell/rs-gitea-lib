
/// EditHookOption options when modify one hook
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditHookOption {
    pub active: Option<bool>,
    pub branch_filter: Option<String>,
    pub config: Option<std::collections::BTreeMap<String, String>>,
    pub events: Option<Vec<String>>,
}

impl EditHookOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditHookOptionBuilder {
        EditHookOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn org_edit_hook() -> EditHookOptionPatchBuilder<crate::generics::MissingOrg, crate::generics::MissingId> {
        EditHookOptionPatchBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_edit_hook() -> EditHookOptionPatchBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        EditHookOptionPatchBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<EditHookOption> for EditHookOptionBuilder {
    fn into(self) -> EditHookOption {
        self.body
    }
}

impl Into<EditHookOption> for EditHookOptionPatchBuilder<crate::generics::OrgExists, crate::generics::IdExists> {
    fn into(self) -> EditHookOption {
        self.inner.body
    }
}

impl Into<EditHookOption> for EditHookOptionPatchBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    fn into(self) -> EditHookOption {
        self.inner.body
    }
}

/// Builder for [`EditHookOption`](./struct.EditHookOption.html) object.
#[derive(Debug, Clone)]
pub struct EditHookOptionBuilder {
    body: self::EditHookOption,
}

impl EditHookOptionBuilder {
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
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> Self {
        self.body.config = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`EditHookOption::org_edit_hook`](./struct.EditHookOption.html#method.org_edit_hook) method for a `PATCH` operation associated with `EditHookOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditHookOptionPatchBuilder<Org, Id> {
    inner: EditHookOptionPatchBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditHookOptionPatchBuilderContainer {
    body: self::EditHookOption,
    param_org: Option<String>,
    param_id: Option<i64>,
}

impl<Org, Id> EditHookOptionPatchBuilder<Org, Id> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> EditHookOptionPatchBuilder<crate::generics::OrgExists, Id> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the hook to update
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditHookOptionPatchBuilder<Org, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
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
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> Self {
        self.inner.body.config = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditHookOptionPatchBuilder<crate::generics::OrgExists, crate::generics::IdExists> {
    type Output = crate::hook::Hook;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/hooks/{id}", org=self.inner.param_org.as_ref().expect("missing parameter org?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

/// Builder created by [`EditHookOption::repo_edit_hook`](./struct.EditHookOption.html#method.repo_edit_hook) method for a `PATCH` operation associated with `EditHookOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditHookOptionPatchBuilder1<Owner, Repo, Id> {
    inner: EditHookOptionPatchBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditHookOptionPatchBuilder1Container {
    body: self::EditHookOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> EditHookOptionPatchBuilder1<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditHookOptionPatchBuilder1<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditHookOptionPatchBuilder1<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the hook
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditHookOptionPatchBuilder1<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
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
    pub fn config(mut self, value: impl Iterator<Item = (String, impl Into<String>)>) -> Self {
        self.inner.body.config = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    #[inline]
    pub fn events(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.events = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditHookOptionPatchBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = crate::hook::Hook;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
