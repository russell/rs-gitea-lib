#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostUsersUsernameTokensBody {
    pub name: String,
}

impl PostUsersUsernameTokensBody {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PostUsersUsernameTokensBodyBuilder<crate::generics::MissingName> {
        PostUsersUsernameTokensBodyBuilder {
            body: Default::default(),
            _name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_create_token() -> PostUsersUsernameTokensBodyPostBuilder<crate::generics::MissingUsername, crate::generics::MissingName> {
        PostUsersUsernameTokensBodyPostBuilder {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }
}

impl Into<PostUsersUsernameTokensBody> for PostUsersUsernameTokensBodyBuilder<crate::generics::NameExists> {
    fn into(self) -> PostUsersUsernameTokensBody {
        self.body
    }
}

impl Into<PostUsersUsernameTokensBody> for PostUsersUsernameTokensBodyPostBuilder<crate::generics::UsernameExists, crate::generics::NameExists> {
    fn into(self) -> PostUsersUsernameTokensBody {
        self.inner.body
    }
}

/// Builder for [`PostUsersUsernameTokensBody`](./struct.PostUsersUsernameTokensBody.html) object.
#[derive(Debug, Clone)]
pub struct PostUsersUsernameTokensBodyBuilder<Name> {
    body: self::PostUsersUsernameTokensBody,
    _name: core::marker::PhantomData<Name>,
}

impl<Name> PostUsersUsernameTokensBodyBuilder<Name> {
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> PostUsersUsernameTokensBodyBuilder<crate::generics::NameExists> {
        self.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`PostUsersUsernameTokensBody::user_create_token`](./struct.PostUsersUsernameTokensBody.html#method.user_create_token) method for a `POST` operation associated with `PostUsersUsernameTokensBody`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PostUsersUsernameTokensBodyPostBuilder<Username, Name> {
    inner: PostUsersUsernameTokensBodyPostBuilderContainer,
    _param_username: core::marker::PhantomData<Username>,
    _name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct PostUsersUsernameTokensBodyPostBuilderContainer {
    body: self::PostUsersUsernameTokensBody,
    param_username: Option<String>,
}

impl<Username, Name> PostUsersUsernameTokensBodyPostBuilder<Username, Name> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> PostUsersUsernameTokensBodyPostBuilder<crate::generics::UsernameExists, Name> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> PostUsersUsernameTokensBodyPostBuilder<Username, crate::generics::NameExists> {
        self.inner.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PostUsersUsernameTokensBodyPostBuilder<crate::generics::UsernameExists, crate::generics::NameExists> {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/tokens", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}
