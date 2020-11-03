
/// EditTeamOption options for editing a team
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditTeamOption {
    pub can_create_org_repo: Option<bool>,
    pub description: Option<String>,
    pub includes_all_repositories: Option<bool>,
    pub name: String,
    pub permission: Option<crate::edit_team_option::EditTeamOptionPermission>,
    pub units: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum EditTeamOptionPermission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
}
impl Default for EditTeamOptionPermission {
    fn default() -> Self {
        EditTeamOptionPermission::Read
    }
}

impl EditTeamOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditTeamOptionBuilder<crate::generics::MissingName> {
        EditTeamOptionBuilder {
            body: Default::default(),
            _name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_edit_team() -> EditTeamOptionPatchBuilder<crate::generics::MissingId, crate::generics::MissingName> {
        EditTeamOptionPatchBuilder {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }
}

impl Into<EditTeamOption> for EditTeamOptionBuilder<crate::generics::NameExists> {
    fn into(self) -> EditTeamOption {
        self.body
    }
}

impl Into<EditTeamOption> for EditTeamOptionPatchBuilder<crate::generics::IdExists, crate::generics::NameExists> {
    fn into(self) -> EditTeamOption {
        self.inner.body
    }
}

/// Builder for [`EditTeamOption`](./struct.EditTeamOption.html) object.
#[derive(Debug, Clone)]
pub struct EditTeamOptionBuilder<Name> {
    body: self::EditTeamOption,
    _name: core::marker::PhantomData<Name>,
}

impl<Name> EditTeamOptionBuilder<Name> {
    #[inline]
    pub fn can_create_org_repo(mut self, value: impl Into<bool>) -> Self {
        self.body.can_create_org_repo = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn includes_all_repositories(mut self, value: impl Into<bool>) -> Self {
        self.body.includes_all_repositories = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> EditTeamOptionBuilder<crate::generics::NameExists> {
        self.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn permission(mut self, value: crate::edit_team_option::EditTeamOptionPermission) -> Self {
        self.body.permission = Some(value.into());
        self
    }

    #[inline]
    pub fn units(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.units = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`EditTeamOption::org_edit_team`](./struct.EditTeamOption.html#method.org_edit_team) method for a `PATCH` operation associated with `EditTeamOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditTeamOptionPatchBuilder<Id, Name> {
    inner: EditTeamOptionPatchBuilderContainer,
    _param_id: core::marker::PhantomData<Id>,
    _name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct EditTeamOptionPatchBuilderContainer {
    body: self::EditTeamOption,
    param_id: Option<i64>,
}

impl<Id, Name> EditTeamOptionPatchBuilder<Id, Name> {
    /// id of the team to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditTeamOptionPatchBuilder<crate::generics::IdExists, Name> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn can_create_org_repo(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.can_create_org_repo = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn includes_all_repositories(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.includes_all_repositories = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> EditTeamOptionPatchBuilder<Id, crate::generics::NameExists> {
        self.inner.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn permission(mut self, value: crate::edit_team_option::EditTeamOptionPermission) -> Self {
        self.inner.body.permission = Some(value.into());
        self
    }

    #[inline]
    pub fn units(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.units = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditTeamOptionPatchBuilder<crate::generics::IdExists, crate::generics::NameExists> {
    type Output = crate::team::Team;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/teams/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

