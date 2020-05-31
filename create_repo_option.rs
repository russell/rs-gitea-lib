
/// CreateRepoOption options when creating repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateRepoOption {
    /// Whether the repository should be auto-intialized?
    pub auto_init: Option<bool>,
    /// Description of the repository to create
    pub description: Option<String>,
    /// Gitignores to use
    pub gitignores: Option<String>,
    /// Issue Label set to use
    pub issue_labels: Option<String>,
    /// License to use
    pub license: Option<String>,
    /// Name of the repository to create
    pub name: String,
    /// Whether the repository is private
    pub private: Option<bool>,
    /// Readme of the repository to create
    pub readme: Option<String>,
}

impl CreateRepoOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateRepoOptionBuilder<crate::generics::MissingName> {
        CreateRepoOptionBuilder {
            body: Default::default(),
            _name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn admin_create_repo() -> CreateRepoOptionPostBuilder<crate::generics::MissingUsername, crate::generics::MissingName> {
        CreateRepoOptionPostBuilder {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn create_org_repo() -> CreateRepoOptionPostBuilder1<crate::generics::MissingOrg, crate::generics::MissingName> {
        CreateRepoOptionPostBuilder1 {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn create_current_user_repo() -> CreateRepoOptionPostBuilder2<crate::generics::MissingName> {
        CreateRepoOptionPostBuilder2 {
            body: Default::default(),
            _name: core::marker::PhantomData,
        }
    }
}

impl Into<CreateRepoOption> for CreateRepoOptionBuilder<crate::generics::NameExists> {
    fn into(self) -> CreateRepoOption {
        self.body
    }
}

impl Into<CreateRepoOption> for CreateRepoOptionPostBuilder<crate::generics::UsernameExists, crate::generics::NameExists> {
    fn into(self) -> CreateRepoOption {
        self.inner.body
    }
}

impl Into<CreateRepoOption> for CreateRepoOptionPostBuilder1<crate::generics::OrgExists, crate::generics::NameExists> {
    fn into(self) -> CreateRepoOption {
        self.inner.body
    }
}

impl Into<CreateRepoOption> for CreateRepoOptionPostBuilder2<crate::generics::NameExists> {
    fn into(self) -> CreateRepoOption {
        self.body
    }
}

/// Builder for [`CreateRepoOption`](./struct.CreateRepoOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateRepoOptionBuilder<Name> {
    body: self::CreateRepoOption,
    _name: core::marker::PhantomData<Name>,
}

impl<Name> CreateRepoOptionBuilder<Name> {
    /// Whether the repository should be auto-intialized?
    #[inline]
    pub fn auto_init(mut self, value: impl Into<bool>) -> Self {
        self.body.auto_init = Some(value.into());
        self
    }

    /// Description of the repository to create
    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    /// Gitignores to use
    #[inline]
    pub fn gitignores(mut self, value: impl Into<String>) -> Self {
        self.body.gitignores = Some(value.into());
        self
    }

    /// Issue Label set to use
    #[inline]
    pub fn issue_labels(mut self, value: impl Into<String>) -> Self {
        self.body.issue_labels = Some(value.into());
        self
    }

    /// License to use
    #[inline]
    pub fn license(mut self, value: impl Into<String>) -> Self {
        self.body.license = Some(value.into());
        self
    }

    /// Name of the repository to create
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> CreateRepoOptionBuilder<crate::generics::NameExists> {
        self.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Whether the repository is private
    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.body.private = Some(value.into());
        self
    }

    /// Readme of the repository to create
    #[inline]
    pub fn readme(mut self, value: impl Into<String>) -> Self {
        self.body.readme = Some(value.into());
        self
    }
}

/// Builder created by [`CreateRepoOption::admin_create_repo`](./struct.CreateRepoOption.html#method.admin_create_repo) method for a `POST` operation associated with `CreateRepoOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateRepoOptionPostBuilder<Username, Name> {
    inner: CreateRepoOptionPostBuilderContainer,
    _param_username: core::marker::PhantomData<Username>,
    _name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct CreateRepoOptionPostBuilderContainer {
    body: self::CreateRepoOption,
    param_username: Option<String>,
}

impl<Username, Name> CreateRepoOptionPostBuilder<Username, Name> {
    /// username of the user. This user will own the created repository
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> CreateRepoOptionPostBuilder<crate::generics::UsernameExists, Name> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// Whether the repository should be auto-intialized?
    #[inline]
    pub fn auto_init(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.auto_init = Some(value.into());
        self
    }

    /// Description of the repository to create
    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    /// Gitignores to use
    #[inline]
    pub fn gitignores(mut self, value: impl Into<String>) -> Self {
        self.inner.body.gitignores = Some(value.into());
        self
    }

    /// Issue Label set to use
    #[inline]
    pub fn issue_labels(mut self, value: impl Into<String>) -> Self {
        self.inner.body.issue_labels = Some(value.into());
        self
    }

    /// License to use
    #[inline]
    pub fn license(mut self, value: impl Into<String>) -> Self {
        self.inner.body.license = Some(value.into());
        self
    }

    /// Name of the repository to create
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> CreateRepoOptionPostBuilder<Username, crate::generics::NameExists> {
        self.inner.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Whether the repository is private
    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.private = Some(value.into());
        self
    }

    /// Readme of the repository to create
    #[inline]
    pub fn readme(mut self, value: impl Into<String>) -> Self {
        self.inner.body.readme = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateRepoOptionPostBuilder<crate::generics::UsernameExists, crate::generics::NameExists> {
    type Output = crate::repository::Repository;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/admin/users/{username}/repos", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::repository::Repository, CreateRepoOptionPostBuilder<crate::generics::UsernameExists, crate::generics::NameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`CreateRepoOption::create_org_repo`](./struct.CreateRepoOption.html#method.create_org_repo) method for a `POST` operation associated with `CreateRepoOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateRepoOptionPostBuilder1<Org, Name> {
    inner: CreateRepoOptionPostBuilder1Container,
    _param_org: core::marker::PhantomData<Org>,
    _name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct CreateRepoOptionPostBuilder1Container {
    body: self::CreateRepoOption,
    param_org: Option<String>,
}

impl<Org, Name> CreateRepoOptionPostBuilder1<Org, Name> {
    /// name of organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> CreateRepoOptionPostBuilder1<crate::generics::OrgExists, Name> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// Whether the repository should be auto-intialized?
    #[inline]
    pub fn auto_init(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.auto_init = Some(value.into());
        self
    }

    /// Description of the repository to create
    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    /// Gitignores to use
    #[inline]
    pub fn gitignores(mut self, value: impl Into<String>) -> Self {
        self.inner.body.gitignores = Some(value.into());
        self
    }

    /// Issue Label set to use
    #[inline]
    pub fn issue_labels(mut self, value: impl Into<String>) -> Self {
        self.inner.body.issue_labels = Some(value.into());
        self
    }

    /// License to use
    #[inline]
    pub fn license(mut self, value: impl Into<String>) -> Self {
        self.inner.body.license = Some(value.into());
        self
    }

    /// Name of the repository to create
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> CreateRepoOptionPostBuilder1<Org, crate::generics::NameExists> {
        self.inner.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Whether the repository is private
    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.private = Some(value.into());
        self
    }

    /// Readme of the repository to create
    #[inline]
    pub fn readme(mut self, value: impl Into<String>) -> Self {
        self.inner.body.readme = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateRepoOptionPostBuilder1<crate::generics::OrgExists, crate::generics::NameExists> {
    type Output = crate::repository::Repository;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/org/{org}/repos", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::repository::Repository, CreateRepoOptionPostBuilder1<crate::generics::OrgExists, crate::generics::NameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`CreateRepoOption::create_current_user_repo`](./struct.CreateRepoOption.html#method.create_current_user_repo) method for a `POST` operation associated with `CreateRepoOption`.
#[derive(Debug, Clone)]
pub struct CreateRepoOptionPostBuilder2<Name> {
    body: self::CreateRepoOption,
    _name: core::marker::PhantomData<Name>,
}

impl<Name> CreateRepoOptionPostBuilder2<Name> {
    /// Whether the repository should be auto-intialized?
    #[inline]
    pub fn auto_init(mut self, value: impl Into<bool>) -> Self {
        self.body.auto_init = Some(value.into());
        self
    }

    /// Description of the repository to create
    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    /// Gitignores to use
    #[inline]
    pub fn gitignores(mut self, value: impl Into<String>) -> Self {
        self.body.gitignores = Some(value.into());
        self
    }

    /// Issue Label set to use
    #[inline]
    pub fn issue_labels(mut self, value: impl Into<String>) -> Self {
        self.body.issue_labels = Some(value.into());
        self
    }

    /// License to use
    #[inline]
    pub fn license(mut self, value: impl Into<String>) -> Self {
        self.body.license = Some(value.into());
        self
    }

    /// Name of the repository to create
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> CreateRepoOptionPostBuilder2<crate::generics::NameExists> {
        self.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Whether the repository is private
    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.body.private = Some(value.into());
        self
    }

    /// Readme of the repository to create
    #[inline]
    pub fn readme(mut self, value: impl Into<String>) -> Self {
        self.body.readme = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateRepoOptionPostBuilder2<crate::generics::NameExists> {
    type Output = crate::repository::Repository;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/repos".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}

impl crate::client::ResponseWrapper<crate::repository::Repository, CreateRepoOptionPostBuilder2<crate::generics::NameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
