
/// Organization represents an organization
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub full_name: Option<String>,
    pub id: Option<i64>,
    pub location: Option<String>,
    pub repo_admin_change_team_access: Option<bool>,
    pub username: Option<String>,
    pub visibility: Option<String>,
    pub website: Option<String>,
}

impl Organization {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OrganizationBuilder {
        OrganizationBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn admin_get_all_orgs() -> OrganizationGetBuilder {
        OrganizationGetBuilder {
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn org_get_all() -> OrganizationGetBuilder1 {
        OrganizationGetBuilder1 {
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn org_get() -> OrganizationGetBuilder2<crate::generics::MissingOrg> {
        OrganizationGetBuilder2 {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_list_current_user_orgs() -> OrganizationGetBuilder3 {
        OrganizationGetBuilder3 {
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn org_list_user_orgs() -> OrganizationGetBuilder4<crate::generics::MissingUsername> {
        OrganizationGetBuilder4 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }
}

impl Into<Organization> for OrganizationBuilder {
    fn into(self) -> Organization {
        self.body
    }
}

/// Builder for [`Organization`](./struct.Organization.html) object.
#[derive(Debug, Clone)]
pub struct OrganizationBuilder {
    body: self::Organization,
}

impl OrganizationBuilder {
    #[inline]
    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.body.avatar_url = Some(value.into());
        self
    }

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
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
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

    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.body.username = Some(value.into());
        self
    }

    #[inline]
    pub fn visibility(mut self, value: impl Into<String>) -> Self {
        self.body.visibility = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.body.website = Some(value.into());
        self
    }
}

/// Builder created by [`Organization::admin_get_all_orgs`](./struct.Organization.html#method.admin_get_all_orgs) method for a `GET` operation associated with `Organization`.
#[derive(Debug, Clone)]
pub struct OrganizationGetBuilder {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl OrganizationGetBuilder {
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for OrganizationGetBuilder {
    type Output = Vec<Organization>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/admin/orgs".into()
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

impl crate::client::ResponseWrapper<Vec<Organization>, OrganizationGetBuilder> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`Organization::org_get_all`](./struct.Organization.html#method.org_get_all) method for a `GET` operation associated with `Organization`.
#[derive(Debug, Clone)]
pub struct OrganizationGetBuilder1 {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl OrganizationGetBuilder1 {
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for OrganizationGetBuilder1 {
    type Output = Vec<Organization>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/orgs".into()
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

/// Builder created by [`Organization::org_get`](./struct.Organization.html#method.org_get) method for a `GET` operation associated with `Organization`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct OrganizationGetBuilder2<Org> {
    inner: OrganizationGetBuilder2Container,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct OrganizationGetBuilder2Container {
    param_org: Option<String>,
}

impl<Org> OrganizationGetBuilder2<Org> {
    /// name of the organization to get
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> OrganizationGetBuilder2<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for OrganizationGetBuilder2<crate::generics::OrgExists> {
    type Output = Organization;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }
}

/// Builder created by [`Organization::org_list_current_user_orgs`](./struct.Organization.html#method.org_list_current_user_orgs) method for a `GET` operation associated with `Organization`.
#[derive(Debug, Clone)]
pub struct OrganizationGetBuilder3 {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl OrganizationGetBuilder3 {
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for OrganizationGetBuilder3 {
    type Output = Vec<Organization>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/orgs".into()
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

/// Builder created by [`Organization::org_list_user_orgs`](./struct.Organization.html#method.org_list_user_orgs) method for a `GET` operation associated with `Organization`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct OrganizationGetBuilder4<Username> {
    inner: OrganizationGetBuilder4Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct OrganizationGetBuilder4Container {
    param_username: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Username> OrganizationGetBuilder4<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> OrganizationGetBuilder4<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for OrganizationGetBuilder4<crate::generics::UsernameExists> {
    type Output = Vec<Organization>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/orgs", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
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
