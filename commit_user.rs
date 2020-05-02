#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommitUser {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
}

impl CommitUser {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CommitUserBuilder {
        CommitUserBuilder {
            body: Default::default(),
        }
    }
}

impl Into<CommitUser> for CommitUserBuilder {
    fn into(self) -> CommitUser {
        self.body
    }
}

/// Builder for [`CommitUser`](./struct.CommitUser.html) object.
#[derive(Debug, Clone)]
pub struct CommitUserBuilder {
    body: self::CommitUser,
}

impl CommitUserBuilder {
    #[inline]
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.body.date = Some(value.into());
        self
    }

    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.body.email = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }
}
