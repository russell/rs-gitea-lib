
/// GPGKey a user GPG key to sign commit and tag in repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetUserGpgKeysIdResponse {
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

impl GetUserGpgKeysIdResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GetUserGpgKeysIdResponseBuilder {
        GetUserGpgKeysIdResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_current_list_gpg_keys() -> GetUserGpgKeysIdResponseGetBuilder {
        GetUserGpgKeysIdResponseGetBuilder
    }

    #[inline]
    pub fn user_current_get_gpg_key() -> GetUserGpgKeysIdResponseGetBuilder1<crate::generics::MissingId> {
        GetUserGpgKeysIdResponseGetBuilder1 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_gpg_keys() -> GetUserGpgKeysIdResponseGetBuilder2<crate::generics::MissingUsername> {
        GetUserGpgKeysIdResponseGetBuilder2 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }
}

impl Into<GetUserGpgKeysIdResponse> for GetUserGpgKeysIdResponseBuilder {
    fn into(self) -> GetUserGpgKeysIdResponse {
        self.body
    }
}

/// Builder for [`GetUserGpgKeysIdResponse`](./struct.GetUserGpgKeysIdResponse.html) object.
#[derive(Debug, Clone)]
pub struct GetUserGpgKeysIdResponseBuilder {
    body: self::GetUserGpgKeysIdResponse,
}

impl GetUserGpgKeysIdResponseBuilder {
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

/// Builder created by [`GetUserGpgKeysIdResponse::user_current_list_gpg_keys`](./struct.GetUserGpgKeysIdResponse.html#method.user_current_list_gpg_keys) method for a `GET` operation associated with `GetUserGpgKeysIdResponse`.
#[derive(Debug, Clone)]
pub struct GetUserGpgKeysIdResponseGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetUserGpgKeysIdResponseGetBuilder {
    type Output = Vec<GetUserGpgKeysIdResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/gpg_keys".into()
    }
}

/// Builder created by [`GetUserGpgKeysIdResponse::user_current_get_gpg_key`](./struct.GetUserGpgKeysIdResponse.html#method.user_current_get_gpg_key) method for a `GET` operation associated with `GetUserGpgKeysIdResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GetUserGpgKeysIdResponseGetBuilder1<Id> {
    inner: GetUserGpgKeysIdResponseGetBuilder1Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct GetUserGpgKeysIdResponseGetBuilder1Container {
    param_id: Option<i64>,
}

impl<Id> GetUserGpgKeysIdResponseGetBuilder1<Id> {
    /// id of key to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> GetUserGpgKeysIdResponseGetBuilder1<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetUserGpgKeysIdResponseGetBuilder1<crate::generics::IdExists> {
    type Output = GetUserGpgKeysIdResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/user/gpg_keys/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`GetUserGpgKeysIdResponse::user_list_gpg_keys`](./struct.GetUserGpgKeysIdResponse.html#method.user_list_gpg_keys) method for a `GET` operation associated with `GetUserGpgKeysIdResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GetUserGpgKeysIdResponseGetBuilder2<Username> {
    inner: GetUserGpgKeysIdResponseGetBuilder2Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct GetUserGpgKeysIdResponseGetBuilder2Container {
    param_username: Option<String>,
}

impl<Username> GetUserGpgKeysIdResponseGetBuilder2<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> GetUserGpgKeysIdResponseGetBuilder2<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetUserGpgKeysIdResponseGetBuilder2<crate::generics::UsernameExists> {
    type Output = Vec<GetUserGpgKeysIdResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/gpg_keys", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }
}
