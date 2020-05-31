
/// Team represents a team in an organization
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Team {
    pub can_create_org_repo: Option<bool>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub includes_all_repositories: Option<bool>,
    pub name: Option<String>,
    pub organization: Option<crate::organization::Organization>,
    pub permission: Option<crate::team::TeamPermission>,
    pub units: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TeamPermission {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "owner")]
    Owner,
}
impl Default for TeamPermission {
    fn default() -> Self {
        TeamPermission::None
    }
}

impl Team {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TeamBuilder {
        TeamBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn org_list_teams() -> TeamGetBuilder<crate::generics::MissingOrg> {
        TeamGetBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_get_team() -> TeamGetBuilder1<crate::generics::MissingId> {
        TeamGetBuilder1 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_teams() -> TeamGetBuilder2 {
        TeamGetBuilder2
    }
}

impl Into<Team> for TeamBuilder {
    fn into(self) -> Team {
        self.body
    }
}

/// Builder for [`Team`](./struct.Team.html) object.
#[derive(Debug, Clone)]
pub struct TeamBuilder {
    body: self::Team,
}

impl TeamBuilder {
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
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn includes_all_repositories(mut self, value: impl Into<bool>) -> Self {
        self.body.includes_all_repositories = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn organization(mut self, value: crate::organization::Organization) -> Self {
        self.body.organization = Some(value.into());
        self
    }

    #[inline]
    pub fn permission(mut self, value: crate::team::TeamPermission) -> Self {
        self.body.permission = Some(value.into());
        self
    }

    #[inline]
    pub fn units(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.units = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`Team::org_list_teams`](./struct.Team.html#method.org_list_teams) method for a `GET` operation associated with `Team`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TeamGetBuilder<Org> {
    inner: TeamGetBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct TeamGetBuilderContainer {
    param_org: Option<String>,
}

impl<Org> TeamGetBuilder<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> TeamGetBuilder<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TeamGetBuilder<crate::generics::OrgExists> {
    type Output = Vec<Team>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/teams", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }
}

/// Builder created by [`Team::org_get_team`](./struct.Team.html#method.org_get_team) method for a `GET` operation associated with `Team`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TeamGetBuilder1<Id> {
    inner: TeamGetBuilder1Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct TeamGetBuilder1Container {
    param_id: Option<i64>,
}

impl<Id> TeamGetBuilder1<Id> {
    /// id of the team to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> TeamGetBuilder1<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TeamGetBuilder1<crate::generics::IdExists> {
    type Output = Team;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/teams/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`Team::user_list_teams`](./struct.Team.html#method.user_list_teams) method for a `GET` operation associated with `Team`.
#[derive(Debug, Clone)]
pub struct TeamGetBuilder2;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TeamGetBuilder2 {
    type Output = Vec<Team>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/teams".into()
    }
}

