
/// CreateTeamOption options for creating a team
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateTeamOption {
    pub can_create_org_repo: Option<bool>,
    pub description: Option<String>,
    pub includes_all_repositories: Option<bool>,
    pub name: String,
    pub permission: Option<crate::create_team_option::CreateTeamOptionPermission>,
    pub units: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CreateTeamOptionPermission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
}
impl Default for CreateTeamOptionPermission {
    fn default() -> Self {
        CreateTeamOptionPermission::Read
    }
}

impl CreateTeamOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateTeamOptionBuilder<crate::generics::MissingName> {
        CreateTeamOptionBuilder {
            body: Default::default(),
            _name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_create_team() -> CreateTeamOptionPostBuilder<crate::generics::MissingOrg, crate::generics::MissingName> {
        CreateTeamOptionPostBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }
}

impl Into<CreateTeamOption> for CreateTeamOptionBuilder<crate::generics::NameExists> {
    fn into(self) -> CreateTeamOption {
        self.body
    }
}

impl Into<CreateTeamOption> for CreateTeamOptionPostBuilder<crate::generics::OrgExists, crate::generics::NameExists> {
    fn into(self) -> CreateTeamOption {
        self.inner.body
    }
}

/// Builder for [`CreateTeamOption`](./struct.CreateTeamOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateTeamOptionBuilder<Name> {
    body: self::CreateTeamOption,
    _name: core::marker::PhantomData<Name>,
}

impl<Name> CreateTeamOptionBuilder<Name> {
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
    pub fn name(mut self, value: impl Into<String>) -> CreateTeamOptionBuilder<crate::generics::NameExists> {
        self.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn permission(mut self, value: crate::create_team_option::CreateTeamOptionPermission) -> Self {
        self.body.permission = Some(value.into());
        self
    }

    #[inline]
    pub fn units(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.units = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`CreateTeamOption::org_create_team`](./struct.CreateTeamOption.html#method.org_create_team) method for a `POST` operation associated with `CreateTeamOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateTeamOptionPostBuilder<Org, Name> {
    inner: CreateTeamOptionPostBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
    _name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct CreateTeamOptionPostBuilderContainer {
    body: self::CreateTeamOption,
    param_org: Option<String>,
}

impl<Org, Name> CreateTeamOptionPostBuilder<Org, Name> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> CreateTeamOptionPostBuilder<crate::generics::OrgExists, Name> {
        self.inner.param_org = Some(value.into());
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
    pub fn name(mut self, value: impl Into<String>) -> CreateTeamOptionPostBuilder<Org, crate::generics::NameExists> {
        self.inner.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn permission(mut self, value: crate::create_team_option::CreateTeamOptionPermission) -> Self {
        self.inner.body.permission = Some(value.into());
        self
    }

    #[inline]
    pub fn units(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.units = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateTeamOptionPostBuilder<crate::generics::OrgExists, crate::generics::NameExists> {
    type Output = crate::team::Team;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/teams", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::team::Team, CreateTeamOptionPostBuilder<crate::generics::OrgExists, crate::generics::NameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

