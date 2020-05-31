
/// CreateGPGKeyOption options create user GPG key
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateGpgKeyOption {
    /// An armored GPG key to add
    pub armored_public_key: String,
}

impl CreateGpgKeyOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateGpgKeyOptionBuilder<crate::generics::MissingArmoredPublicKey> {
        CreateGpgKeyOptionBuilder {
            body: Default::default(),
            _armored_public_key: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_current_post_gpg_key() -> CreateGpgKeyOptionPostBuilder<crate::generics::MissingArmoredPublicKey> {
        CreateGpgKeyOptionPostBuilder {
            body: Default::default(),
            _armored_public_key: core::marker::PhantomData,
        }
    }
}

impl Into<CreateGpgKeyOption> for CreateGpgKeyOptionBuilder<crate::generics::ArmoredPublicKeyExists> {
    fn into(self) -> CreateGpgKeyOption {
        self.body
    }
}

impl Into<CreateGpgKeyOption> for CreateGpgKeyOptionPostBuilder<crate::generics::ArmoredPublicKeyExists> {
    fn into(self) -> CreateGpgKeyOption {
        self.body
    }
}

/// Builder for [`CreateGpgKeyOption`](./struct.CreateGpgKeyOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateGpgKeyOptionBuilder<ArmoredPublicKey> {
    body: self::CreateGpgKeyOption,
    _armored_public_key: core::marker::PhantomData<ArmoredPublicKey>,
}

impl<ArmoredPublicKey> CreateGpgKeyOptionBuilder<ArmoredPublicKey> {
    /// An armored GPG key to add
    #[inline]
    pub fn armored_public_key(mut self, value: impl Into<String>) -> CreateGpgKeyOptionBuilder<crate::generics::ArmoredPublicKeyExists> {
        self.body.armored_public_key = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`CreateGpgKeyOption::user_current_post_gpg_key`](./struct.CreateGpgKeyOption.html#method.user_current_post_gpg_key) method for a `POST` operation associated with `CreateGpgKeyOption`.
#[derive(Debug, Clone)]
pub struct CreateGpgKeyOptionPostBuilder<ArmoredPublicKey> {
    body: self::CreateGpgKeyOption,
    _armored_public_key: core::marker::PhantomData<ArmoredPublicKey>,
}

impl<ArmoredPublicKey> CreateGpgKeyOptionPostBuilder<ArmoredPublicKey> {
    /// An armored GPG key to add
    #[inline]
    pub fn armored_public_key(mut self, value: impl Into<String>) -> CreateGpgKeyOptionPostBuilder<crate::generics::ArmoredPublicKeyExists> {
        self.body.armored_public_key = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateGpgKeyOptionPostBuilder<crate::generics::ArmoredPublicKeyExists> {
    type Output = crate::gpg_key::GpgKey;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/gpg_keys".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}
