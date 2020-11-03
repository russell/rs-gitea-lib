
/// CreateOAuth2ApplicationOptions holds options to create an oauth2 application
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateOAuth2ApplicationOptions {
    pub name: Option<String>,
    pub redirect_uris: Option<Vec<String>>,
}

impl CreateOAuth2ApplicationOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateOAuth2ApplicationOptionsBuilder {
        CreateOAuth2ApplicationOptionsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_create_o_auth2_application() -> CreateOAuth2ApplicationOptionsPostBuilder {
        CreateOAuth2ApplicationOptionsPostBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_update_o_auth2_application() -> CreateOAuth2ApplicationOptionsPatchBuilder1<crate::generics::MissingId> {
        CreateOAuth2ApplicationOptionsPatchBuilder1 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<CreateOAuth2ApplicationOptions> for CreateOAuth2ApplicationOptionsBuilder {
    fn into(self) -> CreateOAuth2ApplicationOptions {
        self.body
    }
}

impl Into<CreateOAuth2ApplicationOptions> for CreateOAuth2ApplicationOptionsPostBuilder {
    fn into(self) -> CreateOAuth2ApplicationOptions {
        self.body
    }
}

impl Into<CreateOAuth2ApplicationOptions> for CreateOAuth2ApplicationOptionsPatchBuilder1<crate::generics::IdExists> {
    fn into(self) -> CreateOAuth2ApplicationOptions {
        self.inner.body
    }
}

/// Builder for [`CreateOAuth2ApplicationOptions`](./struct.CreateOAuth2ApplicationOptions.html) object.
#[derive(Debug, Clone)]
pub struct CreateOAuth2ApplicationOptionsBuilder {
    body: self::CreateOAuth2ApplicationOptions,
}

impl CreateOAuth2ApplicationOptionsBuilder {
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn redirect_uris(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.redirect_uris = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`CreateOAuth2ApplicationOptions::user_create_o_auth2_application`](./struct.CreateOAuth2ApplicationOptions.html#method.user_create_o_auth2_application) method for a `POST` operation associated with `CreateOAuth2ApplicationOptions`.
#[derive(Debug, Clone)]
pub struct CreateOAuth2ApplicationOptionsPostBuilder {
    body: self::CreateOAuth2ApplicationOptions,
}

impl CreateOAuth2ApplicationOptionsPostBuilder {
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn redirect_uris(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.redirect_uris = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateOAuth2ApplicationOptionsPostBuilder {
    type Output = crate::o_auth2_application::OAuth2Application;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/applications/oauth2".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}

/// Builder created by [`CreateOAuth2ApplicationOptions::user_update_o_auth2_application`](./struct.CreateOAuth2ApplicationOptions.html#method.user_update_o_auth2_application) method for a `PATCH` operation associated with `CreateOAuth2ApplicationOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateOAuth2ApplicationOptionsPatchBuilder1<Id> {
    inner: CreateOAuth2ApplicationOptionsPatchBuilder1Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct CreateOAuth2ApplicationOptionsPatchBuilder1Container {
    body: self::CreateOAuth2ApplicationOptions,
    param_id: Option<i64>,
}

impl<Id> CreateOAuth2ApplicationOptionsPatchBuilder1<Id> {
    /// application to be updated
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> CreateOAuth2ApplicationOptionsPatchBuilder1<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn redirect_uris(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.redirect_uris = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateOAuth2ApplicationOptionsPatchBuilder1<crate::generics::IdExists> {
    type Output = crate::o_auth2_application::OAuth2Application;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/user/applications/oauth2/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
