
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
    pub fn repo_list_teams() -> TeamGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        TeamGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_check_team() -> TeamGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingTeam> {
        TeamGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_team: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_get_team() -> TeamGetBuilder3<crate::generics::MissingId> {
        TeamGetBuilder3 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_teams() -> TeamGetBuilder4 {
        TeamGetBuilder4 {
            param_page: None,
            param_limit: None,
        }
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
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Org> TeamGetBuilder<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> TeamGetBuilder<crate::generics::OrgExists> {
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TeamGetBuilder<crate::generics::OrgExists> {
    type Output = Vec<Team>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/teams", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
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

/// Builder created by [`Team::repo_list_teams`](./struct.Team.html#method.repo_list_teams) method for a `GET` operation associated with `Team`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TeamGetBuilder1<Owner, Repo> {
    inner: TeamGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct TeamGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> TeamGetBuilder1<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TeamGetBuilder1<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TeamGetBuilder1<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TeamGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Team>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/teams", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`Team::repo_check_team`](./struct.Team.html#method.repo_check_team) method for a `GET` operation associated with `Team`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TeamGetBuilder2<Owner, Repo, Team> {
    inner: TeamGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_team: core::marker::PhantomData<Team>,
}

#[derive(Debug, Default, Clone)]
struct TeamGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_team: Option<String>,
}

impl<Owner, Repo, Team> TeamGetBuilder2<Owner, Repo, Team> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TeamGetBuilder2<crate::generics::OwnerExists, Repo, Team> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TeamGetBuilder2<Owner, crate::generics::RepoExists, Team> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// team name
    #[inline]
    pub fn team(mut self, value: impl Into<String>) -> TeamGetBuilder2<Owner, Repo, crate::generics::TeamExists> {
        self.inner.param_team = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TeamGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::TeamExists> {
    type Output = Team;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/teams/{team}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), team=self.inner.param_team.as_ref().expect("missing parameter team?")).into()
    }
}

impl crate::client::ResponseWrapper<Team, TeamGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::TeamExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`Team::org_get_team`](./struct.Team.html#method.org_get_team) method for a `GET` operation associated with `Team`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TeamGetBuilder3<Id> {
    inner: TeamGetBuilder3Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct TeamGetBuilder3Container {
    param_id: Option<i64>,
}

impl<Id> TeamGetBuilder3<Id> {
    /// id of the team to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> TeamGetBuilder3<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TeamGetBuilder3<crate::generics::IdExists> {
    type Output = Team;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/teams/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`Team::user_list_teams`](./struct.Team.html#method.user_list_teams) method for a `GET` operation associated with `Team`.
#[derive(Debug, Clone)]
pub struct TeamGetBuilder4 {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl TeamGetBuilder4 {
    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TeamGetBuilder4 {
    type Output = Vec<Team>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/teams".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

