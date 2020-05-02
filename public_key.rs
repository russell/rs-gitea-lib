
/// PublicKey publickey is a user key to push code to repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub created_at: Option<String>,
    pub fingerprint: Option<String>,
    pub id: Option<i64>,
    pub key: Option<String>,
    pub key_type: Option<String>,
    pub read_only: Option<bool>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub user: Option<crate::user::User>,
}

impl PublicKey {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PublicKeyBuilder {
        PublicKeyBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_current_list_keys() -> PublicKeyGetBuilder {
        PublicKeyGetBuilder {
            param_fingerprint: None,
        }
    }

    #[inline]
    pub fn user_current_get_key() -> PublicKeyGetBuilder1<crate::generics::MissingId> {
        PublicKeyGetBuilder1 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_keys() -> PublicKeyGetBuilder2<crate::generics::MissingUsername> {
        PublicKeyGetBuilder2 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }
}

impl Into<PublicKey> for PublicKeyBuilder {
    fn into(self) -> PublicKey {
        self.body
    }
}

/// Builder for [`PublicKey`](./struct.PublicKey.html) object.
#[derive(Debug, Clone)]
pub struct PublicKeyBuilder {
    body: self::PublicKey,
}

impl PublicKeyBuilder {
    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.body.fingerprint = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.body.key = Some(value.into());
        self
    }

    #[inline]
    pub fn key_type(mut self, value: impl Into<String>) -> Self {
        self.body.key_type = Some(value.into());
        self
    }

    #[inline]
    pub fn read_only(mut self, value: impl Into<bool>) -> Self {
        self.body.read_only = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.body.title = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }

    #[inline]
    pub fn user(mut self, value: crate::user::User) -> Self {
        self.body.user = Some(value.into());
        self
    }
}

/// Builder created by [`PublicKey::user_current_list_keys`](./struct.PublicKey.html#method.user_current_list_keys) method for a `GET` operation associated with `PublicKey`.
#[derive(Debug, Clone)]
pub struct PublicKeyGetBuilder {
    param_fingerprint: Option<String>,
}

impl PublicKeyGetBuilder {
    /// fingerprint of the key
    #[inline]
    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.param_fingerprint = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PublicKeyGetBuilder {
    type Output = Vec<PublicKey>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/keys".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("fingerprint", self.param_fingerprint.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`PublicKey::user_current_get_key`](./struct.PublicKey.html#method.user_current_get_key) method for a `GET` operation associated with `PublicKey`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PublicKeyGetBuilder1<Id> {
    inner: PublicKeyGetBuilder1Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct PublicKeyGetBuilder1Container {
    param_id: Option<i64>,
}

impl<Id> PublicKeyGetBuilder1<Id> {
    /// id of key to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PublicKeyGetBuilder1<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PublicKeyGetBuilder1<crate::generics::IdExists> {
    type Output = PublicKey;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/user/keys/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`PublicKey::user_list_keys`](./struct.PublicKey.html#method.user_list_keys) method for a `GET` operation associated with `PublicKey`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PublicKeyGetBuilder2<Username> {
    inner: PublicKeyGetBuilder2Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct PublicKeyGetBuilder2Container {
    param_username: Option<String>,
    param_fingerprint: Option<String>,
}

impl<Username> PublicKeyGetBuilder2<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> PublicKeyGetBuilder2<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// fingerprint of the key
    #[inline]
    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.inner.param_fingerprint = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PublicKeyGetBuilder2<crate::generics::UsernameExists> {
    type Output = Vec<PublicKey>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/keys", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("fingerprint", self.inner.param_fingerprint.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
