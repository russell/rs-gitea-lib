
/// CreateOrgOption options for creating an organization
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateOrgOption {
    pub description: Option<String>,
    pub full_name: Option<String>,
    pub location: Option<String>,
    pub repo_admin_change_team_access: Option<bool>,
    pub username: String,
    /// possible values are `public` (default), `limited` or `private`
    pub visibility: Option<crate::create_org_option::CreateOrgOptionVisibility>,
    pub website: Option<String>,
}

/// possible values are `public` (default), `limited` or `private`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CreateOrgOptionVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "private")]
    Private,
}
impl Default for CreateOrgOptionVisibility {
    fn default() -> Self {
        CreateOrgOptionVisibility::Public
    }
}

impl CreateOrgOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateOrgOptionBuilder<crate::generics::MissingUsername> {
        CreateOrgOptionBuilder {
            body: Default::default(),
            _username: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn admin_create_org() -> CreateOrgOptionPostBuilder<crate::generics::MissingUsername> {
        CreateOrgOptionPostBuilder {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_create() -> CreateOrgOptionPostBuilder1<crate::generics::MissingUsername> {
        CreateOrgOptionPostBuilder1 {
            body: Default::default(),
            _username: core::marker::PhantomData,
        }
    }
}

impl Into<CreateOrgOption> for CreateOrgOptionBuilder<crate::generics::UsernameExists> {
    fn into(self) -> CreateOrgOption {
        self.body
    }
}

impl Into<CreateOrgOption> for CreateOrgOptionPostBuilder<crate::generics::UsernameExists> {
    fn into(self) -> CreateOrgOption {
        self.inner.body
    }
}

impl Into<CreateOrgOption> for CreateOrgOptionPostBuilder1<crate::generics::UsernameExists> {
    fn into(self) -> CreateOrgOption {
        self.body
    }
}

/// Builder for [`CreateOrgOption`](./struct.CreateOrgOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateOrgOptionBuilder<Username> {
    body: self::CreateOrgOption,
    _username: core::marker::PhantomData<Username>,
}

impl<Username> CreateOrgOptionBuilder<Username> {
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

    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> CreateOrgOptionBuilder<crate::generics::UsernameExists> {
        self.body.username = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// possible values are `public` (default), `limited` or `private`
    #[inline]
    pub fn visibility(mut self, value: crate::create_org_option::CreateOrgOptionVisibility) -> Self {
        self.body.visibility = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.body.website = Some(value.into());
        self
    }
}

/// Builder created by [`CreateOrgOption::admin_create_org`](./struct.CreateOrgOption.html#method.admin_create_org) method for a `POST` operation associated with `CreateOrgOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateOrgOptionPostBuilder<Username> {
    inner: CreateOrgOptionPostBuilderContainer,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct CreateOrgOptionPostBuilderContainer {
    body: self::CreateOrgOption,
    param_username: Option<String>,
}

impl<Username> CreateOrgOptionPostBuilder<Username> {
    /// username of the user that will own the created organization
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> CreateOrgOptionPostBuilder<crate::generics::UsernameExists> {
        self.inner.param_username = Some({
            let val = value.into();
            self.inner.body.username = val.clone().into();
            val
        });
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

    /// possible values are `public` (default), `limited` or `private`
    #[inline]
    pub fn visibility(mut self, value: crate::create_org_option::CreateOrgOptionVisibility) -> Self {
        self.inner.body.visibility = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.inner.body.website = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateOrgOptionPostBuilder<crate::generics::UsernameExists> {
    type Output = crate::organization::Organization;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/admin/users/{username}/orgs", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::organization::Organization, CreateOrgOptionPostBuilder<crate::generics::UsernameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`CreateOrgOption::org_create`](./struct.CreateOrgOption.html#method.org_create) method for a `POST` operation associated with `CreateOrgOption`.
#[derive(Debug, Clone)]
pub struct CreateOrgOptionPostBuilder1<Username> {
    body: self::CreateOrgOption,
    _username: core::marker::PhantomData<Username>,
}

impl<Username> CreateOrgOptionPostBuilder1<Username> {
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

    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> CreateOrgOptionPostBuilder1<crate::generics::UsernameExists> {
        self.body.username = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// possible values are `public` (default), `limited` or `private`
    #[inline]
    pub fn visibility(mut self, value: crate::create_org_option::CreateOrgOptionVisibility) -> Self {
        self.body.visibility = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.body.website = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateOrgOptionPostBuilder1<crate::generics::UsernameExists> {
    type Output = crate::organization::Organization;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/orgs".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}

impl crate::client::ResponseWrapper<crate::organization::Organization, CreateOrgOptionPostBuilder1<crate::generics::UsernameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

