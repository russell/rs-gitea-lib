
/// CreateUserOption create user options
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateUserOption {
    pub email: String,
    pub full_name: Option<String>,
    pub login_name: Option<String>,
    pub must_change_password: Option<bool>,
    pub password: String,
    pub send_notify: Option<bool>,
    pub source_id: Option<i64>,
    pub username: String,
}

impl CreateUserOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateUserOptionBuilder<crate::generics::MissingEmail, crate::generics::MissingPassword, crate::generics::MissingUsername> {
        CreateUserOptionBuilder {
            body: Default::default(),
            _email: core::marker::PhantomData,
            _password: core::marker::PhantomData,
            _username: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn admin_create_user() -> CreateUserOptionPostBuilder<crate::generics::MissingEmail, crate::generics::MissingPassword, crate::generics::MissingUsername> {
        CreateUserOptionPostBuilder {
            body: Default::default(),
            _email: core::marker::PhantomData,
            _password: core::marker::PhantomData,
            _username: core::marker::PhantomData,
        }
    }
}

impl Into<CreateUserOption> for CreateUserOptionBuilder<crate::generics::EmailExists, crate::generics::PasswordExists, crate::generics::UsernameExists> {
    fn into(self) -> CreateUserOption {
        self.body
    }
}

impl Into<CreateUserOption> for CreateUserOptionPostBuilder<crate::generics::EmailExists, crate::generics::PasswordExists, crate::generics::UsernameExists> {
    fn into(self) -> CreateUserOption {
        self.body
    }
}

/// Builder for [`CreateUserOption`](./struct.CreateUserOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateUserOptionBuilder<Email, Password, Username> {
    body: self::CreateUserOption,
    _email: core::marker::PhantomData<Email>,
    _password: core::marker::PhantomData<Password>,
    _username: core::marker::PhantomData<Username>,
}

impl<Email, Password, Username> CreateUserOptionBuilder<Email, Password, Username> {
    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> CreateUserOptionBuilder<crate::generics::EmailExists, Password, Username> {
        self.body.email = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.body.full_name = Some(value.into());
        self
    }

    #[inline]
    pub fn login_name(mut self, value: impl Into<String>) -> Self {
        self.body.login_name = Some(value.into());
        self
    }

    #[inline]
    pub fn must_change_password(mut self, value: impl Into<bool>) -> Self {
        self.body.must_change_password = Some(value.into());
        self
    }

    #[inline]
    pub fn password(mut self, value: impl Into<String>) -> CreateUserOptionBuilder<Email, crate::generics::PasswordExists, Username> {
        self.body.password = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn send_notify(mut self, value: impl Into<bool>) -> Self {
        self.body.send_notify = Some(value.into());
        self
    }

    #[inline]
    pub fn source_id(mut self, value: impl Into<i64>) -> Self {
        self.body.source_id = Some(value.into());
        self
    }

    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> CreateUserOptionBuilder<Email, Password, crate::generics::UsernameExists> {
        self.body.username = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`CreateUserOption::admin_create_user`](./struct.CreateUserOption.html#method.admin_create_user) method for a `POST` operation associated with `CreateUserOption`.
#[derive(Debug, Clone)]
pub struct CreateUserOptionPostBuilder<Email, Password, Username> {
    body: self::CreateUserOption,
    _email: core::marker::PhantomData<Email>,
    _password: core::marker::PhantomData<Password>,
    _username: core::marker::PhantomData<Username>,
}

impl<Email, Password, Username> CreateUserOptionPostBuilder<Email, Password, Username> {
    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> CreateUserOptionPostBuilder<crate::generics::EmailExists, Password, Username> {
        self.body.email = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.body.full_name = Some(value.into());
        self
    }

    #[inline]
    pub fn login_name(mut self, value: impl Into<String>) -> Self {
        self.body.login_name = Some(value.into());
        self
    }

    #[inline]
    pub fn must_change_password(mut self, value: impl Into<bool>) -> Self {
        self.body.must_change_password = Some(value.into());
        self
    }

    #[inline]
    pub fn password(mut self, value: impl Into<String>) -> CreateUserOptionPostBuilder<Email, crate::generics::PasswordExists, Username> {
        self.body.password = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn send_notify(mut self, value: impl Into<bool>) -> Self {
        self.body.send_notify = Some(value.into());
        self
    }

    #[inline]
    pub fn source_id(mut self, value: impl Into<i64>) -> Self {
        self.body.source_id = Some(value.into());
        self
    }

    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> CreateUserOptionPostBuilder<Email, Password, crate::generics::UsernameExists> {
        self.body.username = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateUserOptionPostBuilder<crate::generics::EmailExists, crate::generics::PasswordExists, crate::generics::UsernameExists> {
    type Output = crate::user::User;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/admin/users".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}

impl crate::client::ResponseWrapper<crate::user::User, CreateUserOptionPostBuilder<crate::generics::EmailExists, crate::generics::PasswordExists, crate::generics::UsernameExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
