
/// GPGKey a user GPG key to sign commit and tag in repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GpgKey {
    pub can_certify: Option<bool>,
    pub can_encrypt_comms: Option<bool>,
    pub can_encrypt_storage: Option<bool>,
    pub can_sign: Option<bool>,
    pub created_at: Option<String>,
    pub emails: Option<Vec<crate::gpg_key_email::GpgKeyEmail>>,
    pub expires_at: Option<String>,
    pub id: Option<i64>,
    pub key_id: Option<String>,
    pub primary_key_id: Option<String>,
    pub public_key: Option<String>,
    pub subkeys: Option<Vec<String>>,
}

impl GpgKey {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GpgKeyBuilder {
        GpgKeyBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_current_list_gpg_keys() -> GpgKeyGetBuilder {
        GpgKeyGetBuilder
    }

    #[inline]
    pub fn user_current_get_gpg_key() -> GpgKeyGetBuilder1<crate::generics::MissingId> {
        GpgKeyGetBuilder1 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_gpg_keys() -> GpgKeyGetBuilder2<crate::generics::MissingUsername> {
        GpgKeyGetBuilder2 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }
}

impl Into<GpgKey> for GpgKeyBuilder {
    fn into(self) -> GpgKey {
        self.body
    }
}

/// Builder for [`GpgKey`](./struct.GpgKey.html) object.
#[derive(Debug, Clone)]
pub struct GpgKeyBuilder {
    body: self::GpgKey,
}

impl GpgKeyBuilder {
    #[inline]
    pub fn can_certify(mut self, value: impl Into<bool>) -> Self {
        self.body.can_certify = Some(value.into());
        self
    }

    #[inline]
    pub fn can_encrypt_comms(mut self, value: impl Into<bool>) -> Self {
        self.body.can_encrypt_comms = Some(value.into());
        self
    }

    #[inline]
    pub fn can_encrypt_storage(mut self, value: impl Into<bool>) -> Self {
        self.body.can_encrypt_storage = Some(value.into());
        self
    }

    #[inline]
    pub fn can_sign(mut self, value: impl Into<bool>) -> Self {
        self.body.can_sign = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn emails(mut self, value: impl Iterator<Item = crate::gpg_key_email::GpgKeyEmail>) -> Self {
        self.body.emails = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.body.expires_at = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn key_id(mut self, value: impl Into<String>) -> Self {
        self.body.key_id = Some(value.into());
        self
    }

    #[inline]
    pub fn primary_key_id(mut self, value: impl Into<String>) -> Self {
        self.body.primary_key_id = Some(value.into());
        self
    }

    #[inline]
    pub fn public_key(mut self, value: impl Into<String>) -> Self {
        self.body.public_key = Some(value.into());
        self
    }

    #[inline]
    pub fn subkeys(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.subkeys = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`GpgKey::user_current_list_gpg_keys`](./struct.GpgKey.html#method.user_current_list_gpg_keys) method for a `GET` operation associated with `GpgKey`.
#[derive(Debug, Clone)]
pub struct GpgKeyGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GpgKeyGetBuilder {
    type Output = Vec<GpgKey>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/gpg_keys".into()
    }
}

/// Builder created by [`GpgKey::user_current_get_gpg_key`](./struct.GpgKey.html#method.user_current_get_gpg_key) method for a `GET` operation associated with `GpgKey`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GpgKeyGetBuilder1<Id> {
    inner: GpgKeyGetBuilder1Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct GpgKeyGetBuilder1Container {
    param_id: Option<i64>,
}

impl<Id> GpgKeyGetBuilder1<Id> {
    /// id of key to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> GpgKeyGetBuilder1<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GpgKeyGetBuilder1<crate::generics::IdExists> {
    type Output = GpgKey;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/user/gpg_keys/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`GpgKey::user_list_gpg_keys`](./struct.GpgKey.html#method.user_list_gpg_keys) method for a `GET` operation associated with `GpgKey`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GpgKeyGetBuilder2<Username> {
    inner: GpgKeyGetBuilder2Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct GpgKeyGetBuilder2Container {
    param_username: Option<String>,
}

impl<Username> GpgKeyGetBuilder2<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> GpgKeyGetBuilder2<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GpgKeyGetBuilder2<crate::generics::UsernameExists> {
    type Output = Vec<GpgKey>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/gpg_keys", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }
}
