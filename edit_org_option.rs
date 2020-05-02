
/// EditOrgOption options for editing an organization
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditOrgOption {
    pub description: Option<String>,
    pub full_name: Option<String>,
    pub location: Option<String>,
    pub repo_admin_change_team_access: Option<bool>,
    /// possible values are `public`, `limited` or `private`
    pub visibility: Option<crate::edit_org_option::EditOrgOptionVisibility>,
    pub website: Option<String>,
}

/// possible values are `public`, `limited` or `private`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum EditOrgOptionVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "private")]
    Private,
}

impl EditOrgOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditOrgOptionBuilder {
        EditOrgOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn org_edit() -> EditOrgOptionPatchBuilder<crate::generics::MissingOrg> {
        EditOrgOptionPatchBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }
}

impl Into<EditOrgOption> for EditOrgOptionBuilder {
    fn into(self) -> EditOrgOption {
        self.body
    }
}

impl Into<EditOrgOption> for EditOrgOptionPatchBuilder<crate::generics::OrgExists> {
    fn into(self) -> EditOrgOption {
        self.inner.body
    }
}

/// Builder for [`EditOrgOption`](./struct.EditOrgOption.html) object.
#[derive(Debug, Clone)]
pub struct EditOrgOptionBuilder {
    body: self::EditOrgOption,
}

impl EditOrgOptionBuilder {
    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.body.full_name = Some(value.into());
        self
    }

    #[inline]
    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.body.location = Some(value.into());
        self
    }

    #[inline]
    pub fn repo_admin_change_team_access(mut self, value: impl Into<bool>) -> Self {
        self.body.repo_admin_change_team_access = Some(value.into());
        self
    }

    /// possible values are `public`, `limited` or `private`
    #[inline]
    pub fn visibility(mut self, value: crate::edit_org_option::EditOrgOptionVisibility) -> Self {
        self.body.visibility = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.body.website = Some(value.into());
        self
    }
}

/// Builder created by [`EditOrgOption::org_edit`](./struct.EditOrgOption.html#method.org_edit) method for a `PATCH` operation associated with `EditOrgOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditOrgOptionPatchBuilder<Org> {
    inner: EditOrgOptionPatchBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct EditOrgOptionPatchBuilderContainer {
    body: self::EditOrgOption,
    param_org: Option<String>,
}

impl<Org> EditOrgOptionPatchBuilder<Org> {
    /// name of the organization to edit
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> EditOrgOptionPatchBuilder<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.full_name = Some(value.into());
        self
    }

    #[inline]
    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.inner.body.location = Some(value.into());
        self
    }

    #[inline]
    pub fn repo_admin_change_team_access(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.repo_admin_change_team_access = Some(value.into());
        self
    }

    /// possible values are `public`, `limited` or `private`
    #[inline]
    pub fn visibility(mut self, value: crate::edit_org_option::EditOrgOptionVisibility) -> Self {
        self.inner.body.visibility = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.inner.body.website = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditOrgOptionPatchBuilder<crate::generics::OrgExists> {
    type Output = crate::post_orgs_response::PostOrgsResponse;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

