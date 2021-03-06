
/// EditLabelOption options for editing a label
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditLabelOption {
    pub color: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
}

impl EditLabelOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditLabelOptionBuilder {
        EditLabelOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn org_edit_label() -> EditLabelOptionPatchBuilder<crate::generics::MissingOrg, crate::generics::MissingId> {
        EditLabelOptionPatchBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_edit_label() -> EditLabelOptionPatchBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        EditLabelOptionPatchBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<EditLabelOption> for EditLabelOptionBuilder {
    fn into(self) -> EditLabelOption {
        self.body
    }
}

impl Into<EditLabelOption> for EditLabelOptionPatchBuilder<crate::generics::OrgExists, crate::generics::IdExists> {
    fn into(self) -> EditLabelOption {
        self.inner.body
    }
}

impl Into<EditLabelOption> for EditLabelOptionPatchBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    fn into(self) -> EditLabelOption {
        self.inner.body
    }
}

/// Builder for [`EditLabelOption`](./struct.EditLabelOption.html) object.
#[derive(Debug, Clone)]
pub struct EditLabelOptionBuilder {
    body: self::EditLabelOption,
}

impl EditLabelOptionBuilder {
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
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }
}

/// Builder created by [`EditLabelOption::org_edit_label`](./struct.EditLabelOption.html#method.org_edit_label) method for a `PATCH` operation associated with `EditLabelOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditLabelOptionPatchBuilder<Org, Id> {
    inner: EditLabelOptionPatchBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditLabelOptionPatchBuilderContainer {
    body: self::EditLabelOption,
    param_org: Option<String>,
    param_id: Option<i64>,
}

impl<Org, Id> EditLabelOptionPatchBuilder<Org, Id> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> EditLabelOptionPatchBuilder<crate::generics::OrgExists, Id> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the label to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditLabelOptionPatchBuilder<Org, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.inner.body.color = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.name = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditLabelOptionPatchBuilder<crate::generics::OrgExists, crate::generics::IdExists> {
    type Output = crate::label::Label;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/labels/{id}", org=self.inner.param_org.as_ref().expect("missing parameter org?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::label::Label, EditLabelOptionPatchBuilder<crate::generics::OrgExists, crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`EditLabelOption::issue_edit_label`](./struct.EditLabelOption.html#method.issue_edit_label) method for a `PATCH` operation associated with `EditLabelOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditLabelOptionPatchBuilder1<Owner, Repo, Id> {
    inner: EditLabelOptionPatchBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditLabelOptionPatchBuilder1Container {
    body: self::EditLabelOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> EditLabelOptionPatchBuilder1<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditLabelOptionPatchBuilder1<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditLabelOptionPatchBuilder1<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the label to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditLabelOptionPatchBuilder1<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.inner.body.color = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.name = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditLabelOptionPatchBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = crate::label::Label;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/labels/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::label::Label, EditLabelOptionPatchBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
