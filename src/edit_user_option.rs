
/// EditUserOption edit user options
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditUserOption {
    pub active: Option<bool>,
    pub admin: Option<bool>,
    pub allow_create_organization: Option<bool>,
    pub allow_git_hook: Option<bool>,
    pub allow_import_local: Option<bool>,
    pub email: String,
    pub full_name: Option<String>,
    pub location: Option<String>,
    pub login_name: Option<String>,
    pub max_repo_creation: Option<i64>,
    pub must_change_password: Option<bool>,
    pub password: Option<String>,
    pub prohibit_login: Option<bool>,
    pub source_id: Option<i64>,
    pub website: Option<String>,
}

impl EditUserOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditUserOptionBuilder<crate::generics::MissingEmail> {
        EditUserOptionBuilder {
            body: Default::default(),
            _email: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn admin_edit_user() -> EditUserOptionPatchBuilder<crate::generics::MissingUsername, crate::generics::MissingEmail> {
        EditUserOptionPatchBuilder {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
            _email: core::marker::PhantomData,
        }
    }
}

impl Into<EditUserOption> for EditUserOptionBuilder<crate::generics::EmailExists> {
    fn into(self) -> EditUserOption {
        self.body
    }
}

impl Into<EditUserOption> for EditUserOptionPatchBuilder<crate::generics::UsernameExists, crate::generics::EmailExists> {
    fn into(self) -> EditUserOption {
        self.inner.body
    }
}

/// Builder for [`EditUserOption`](./struct.EditUserOption.html) object.
#[derive(Debug, Clone)]
pub struct EditUserOptionBuilder<Email> {
    body: self::EditUserOption,
    _email: core::marker::PhantomData<Email>,
}

impl<Email> EditUserOptionBuilder<Email> {
    #[inline]
    pub fn active(mut self, value: impl Into<bool>) -> Self {
        self.body.active = Some(value.into());
        self
    }

    #[inline]
    pub fn admin(mut self, value: impl Into<bool>) -> Self {
        self.body.admin = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_create_organization(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_create_organization = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_git_hook(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_git_hook = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_import_local(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_import_local = Some(value.into());
        self
    }

    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> EditUserOptionBuilder<crate::generics::EmailExists> {
        self.body.email = value.into();
        unsafe { std::mem::transmute(self) }
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
    pub fn login_name(mut self, value: impl Into<String>) -> Self {
        self.body.login_name = Some(value.into());
        self
    }

    #[inline]
    pub fn max_repo_creation(mut self, value: impl Into<i64>) -> Self {
        self.body.max_repo_creation = Some(value.into());
        self
    }

    #[inline]
    pub fn must_change_password(mut self, value: impl Into<bool>) -> Self {
        self.body.must_change_password = Some(value.into());
        self
    }

    #[inline]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.body.password = Some(value.into());
        self
    }

    #[inline]
    pub fn prohibit_login(mut self, value: impl Into<bool>) -> Self {
        self.body.prohibit_login = Some(value.into());
        self
    }

    #[inline]
    pub fn source_id(mut self, value: impl Into<i64>) -> Self {
        self.body.source_id = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.body.website = Some(value.into());
        self
    }
}

/// Builder created by [`EditUserOption::admin_edit_user`](./struct.EditUserOption.html#method.admin_edit_user) method for a `PATCH` operation associated with `EditUserOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditUserOptionPatchBuilder<Username, Email> {
    inner: EditUserOptionPatchBuilderContainer,
    _param_username: core::marker::PhantomData<Username>,
    _email: core::marker::PhantomData<Email>,
}

#[derive(Debug, Default, Clone)]
struct EditUserOptionPatchBuilderContainer {
    body: self::EditUserOption,
    param_username: Option<String>,
}

impl<Username, Email> EditUserOptionPatchBuilder<Username, Email> {
    /// username of user to edit
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> EditUserOptionPatchBuilder<crate::generics::UsernameExists, Email> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn active(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.active = Some(value.into());
        self
    }

    #[inline]
    pub fn admin(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.admin = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_create_organization(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.allow_create_organization = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_git_hook(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.allow_git_hook = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_import_local(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.allow_import_local = Some(value.into());
        self
    }

    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> EditUserOptionPatchBuilder<Username, crate::generics::EmailExists> {
        self.inner.body.email = value.into();
        unsafe { std::mem::transmute(self) }
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
    pub fn login_name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.login_name = Some(value.into());
        self
    }

    #[inline]
    pub fn max_repo_creation(mut self, value: impl Into<i64>) -> Self {
        self.inner.body.max_repo_creation = Some(value.into());
        self
    }

    #[inline]
    pub fn must_change_password(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.must_change_password = Some(value.into());
        self
    }

    #[inline]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.inner.body.password = Some(value.into());
        self
    }

    #[inline]
    pub fn prohibit_login(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.prohibit_login = Some(value.into());
        self
    }

    #[inline]
    pub fn source_id(mut self, value: impl Into<i64>) -> Self {
        self.inner.body.source_id = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.inner.body.website = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditUserOptionPatchBuilder<crate::generics::UsernameExists, crate::generics::EmailExists> {
    type Output = crate::user::User;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/admin/users/{username}", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::user::User, EditUserOptionPatchBuilder<crate::generics::UsernameExists, crate::generics::EmailExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
