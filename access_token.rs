#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub sha1: Option<String>,
    pub token_last_eight: Option<String>,
}

impl AccessToken {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> AccessTokenBuilder {
        AccessTokenBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_get_tokens() -> AccessTokenGetBuilder<crate::generics::MissingUsername> {
        AccessTokenGetBuilder {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }
}

impl Into<AccessToken> for AccessTokenBuilder {
    fn into(self) -> AccessToken {
        self.body
    }
}

/// Builder for [`AccessToken`](./struct.AccessToken.html) object.
#[derive(Debug, Clone)]
pub struct AccessTokenBuilder {
    body: self::AccessToken,
}

impl AccessTokenBuilder {
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn sha1(mut self, value: impl Into<String>) -> Self {
        self.body.sha1 = Some(value.into());
        self
    }

    #[inline]
    pub fn token_last_eight(mut self, value: impl Into<String>) -> Self {
        self.body.token_last_eight = Some(value.into());
        self
    }
}

/// Builder created by [`AccessToken::user_get_tokens`](./struct.AccessToken.html#method.user_get_tokens) method for a `GET` operation associated with `AccessToken`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AccessTokenGetBuilder<Username> {
    inner: AccessTokenGetBuilderContainer,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct AccessTokenGetBuilderContainer {
    param_username: Option<String>,
}

impl<Username> AccessTokenGetBuilder<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> AccessTokenGetBuilder<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for AccessTokenGetBuilder<crate::generics::UsernameExists> {
    type Output = Vec<AccessToken>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/tokens", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }
}
