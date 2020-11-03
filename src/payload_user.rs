
/// PayloadUser represents the author or committer of a commit
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PayloadUser {
    pub email: Option<String>,
    /// Full name of the commit author
    pub name: Option<String>,
    pub username: Option<String>,
}

impl PayloadUser {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PayloadUserBuilder {
        PayloadUserBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PayloadUser> for PayloadUserBuilder {
    fn into(self) -> PayloadUser {
        self.body
    }
}

/// Builder for [`PayloadUser`](./struct.PayloadUser.html) object.
#[derive(Debug, Clone)]
pub struct PayloadUserBuilder {
    body: self::PayloadUser,
}

impl PayloadUserBuilder {
    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.body.email = Some(value.into());
        self
    }

    /// Full name of the commit author
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.body.username = Some(value.into());
        self
    }
}
