
/// Email an email address belonging to a user
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Email {
    pub email: Option<String>,
    pub primary: Option<bool>,
    pub verified: Option<bool>,
}

impl Email {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EmailBuilder {
        EmailBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_list_emails() -> EmailGetBuilder {
        EmailGetBuilder
    }
}

impl Into<Email> for EmailBuilder {
    fn into(self) -> Email {
        self.body
    }
}

/// Builder for [`Email`](./struct.Email.html) object.
#[derive(Debug, Clone)]
pub struct EmailBuilder {
    body: self::Email,
}

impl EmailBuilder {
    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.body.email = Some(value.into());
        self
    }

    #[inline]
    pub fn primary(mut self, value: impl Into<bool>) -> Self {
        self.body.primary = Some(value.into());
        self
    }

    #[inline]
    pub fn verified(mut self, value: impl Into<bool>) -> Self {
        self.body.verified = Some(value.into());
        self
    }
}

/// Builder created by [`Email::user_list_emails`](./struct.Email.html#method.user_list_emails) method for a `GET` operation associated with `Email`.
#[derive(Debug, Clone)]
pub struct EmailGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EmailGetBuilder {
    type Output = Vec<Email>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/emails".into()
    }
}
