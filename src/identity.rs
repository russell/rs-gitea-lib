
/// Identity for a person's identity like an author or committer
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub email: Option<String>,
    pub name: Option<String>,
}

impl Identity {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> IdentityBuilder {
        IdentityBuilder {
            body: Default::default(),
        }
    }
}

impl Into<Identity> for IdentityBuilder {
    fn into(self) -> Identity {
        self.body
    }
}

/// Builder for [`Identity`](./struct.Identity.html) object.
#[derive(Debug, Clone)]
pub struct IdentityBuilder {
    body: self::Identity,
}

impl IdentityBuilder {
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
