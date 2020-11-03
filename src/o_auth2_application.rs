#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OAuth2Application {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub created: Option<String>,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub redirect_uris: Option<Vec<String>>,
}

impl OAuth2Application {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OAuth2ApplicationBuilder {
        OAuth2ApplicationBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_get_oauth2_application() -> OAuth2ApplicationGetBuilder {
        OAuth2ApplicationGetBuilder {
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn user_get_o_auth2_application() -> OAuth2ApplicationGetBuilder1<crate::generics::MissingId> {
        OAuth2ApplicationGetBuilder1 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<OAuth2Application> for OAuth2ApplicationBuilder {
    fn into(self) -> OAuth2Application {
        self.body
    }
}

/// Builder for [`OAuth2Application`](./struct.OAuth2Application.html) object.
#[derive(Debug, Clone)]
pub struct OAuth2ApplicationBuilder {
    body: self::OAuth2Application,
}

impl OAuth2ApplicationBuilder {
    #[inline]
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.body.client_id = Some(value.into());
        self
    }

    #[inline]
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.body.client_secret = Some(value.into());
        self
    }

    #[inline]
    pub fn created(mut self, value: impl Into<String>) -> Self {
        self.body.created = Some(value.into());
        self
    }

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
    pub fn redirect_uris(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.redirect_uris = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`OAuth2Application::user_get_oauth2_application`](./struct.OAuth2Application.html#method.user_get_oauth2_application) method for a `GET` operation associated with `OAuth2Application`.
#[derive(Debug, Clone)]
pub struct OAuth2ApplicationGetBuilder {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl OAuth2ApplicationGetBuilder {
    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for OAuth2ApplicationGetBuilder {
    type Output = Vec<OAuth2Application>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/applications/oauth2".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`OAuth2Application::user_get_o_auth2_application`](./struct.OAuth2Application.html#method.user_get_o_auth2_application) method for a `GET` operation associated with `OAuth2Application`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct OAuth2ApplicationGetBuilder1<Id> {
    inner: OAuth2ApplicationGetBuilder1Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct OAuth2ApplicationGetBuilder1Container {
    param_id: Option<i64>,
}

impl<Id> OAuth2ApplicationGetBuilder1<Id> {
    /// Application ID to be found
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> OAuth2ApplicationGetBuilder1<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for OAuth2ApplicationGetBuilder1<crate::generics::IdExists> {
    type Output = OAuth2Application;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/user/applications/oauth2/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
