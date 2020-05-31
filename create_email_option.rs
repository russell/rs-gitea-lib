
/// CreateEmailOption options when creating email addresses
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateEmailOption {
    /// email addresses to add
    pub emails: Option<Vec<String>>,
}

impl CreateEmailOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateEmailOptionBuilder {
        CreateEmailOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_add_email() -> CreateEmailOptionPostBuilder {
        CreateEmailOptionPostBuilder {
            body: Default::default(),
        }
    }
}

impl Into<CreateEmailOption> for CreateEmailOptionBuilder {
    fn into(self) -> CreateEmailOption {
        self.body
    }
}

impl Into<CreateEmailOption> for CreateEmailOptionPostBuilder {
    fn into(self) -> CreateEmailOption {
        self.body
    }
}

/// Builder for [`CreateEmailOption`](./struct.CreateEmailOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateEmailOptionBuilder {
    body: self::CreateEmailOption,
}

impl CreateEmailOptionBuilder {
    /// email addresses to add
    #[inline]
    pub fn emails(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.emails = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`CreateEmailOption::user_add_email`](./struct.CreateEmailOption.html#method.user_add_email) method for a `POST` operation associated with `CreateEmailOption`.
#[derive(Debug, Clone)]
pub struct CreateEmailOptionPostBuilder {
    body: self::CreateEmailOption,
}

impl CreateEmailOptionPostBuilder {
    /// email addresses to add
    #[inline]
    pub fn emails(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.emails = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateEmailOptionPostBuilder {
    type Output = Vec<crate::email::Email>;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/emails".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}

impl crate::client::ResponseWrapper<Vec<crate::email::Email>, CreateEmailOptionPostBuilder> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
