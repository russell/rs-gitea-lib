
/// GPGKeyEmail an email attached to a GPGKey
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GpgKeyEmail {
    pub email: Option<String>,
    pub verified: Option<bool>,
}

impl GpgKeyEmail {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GpgKeyEmailBuilder {
        GpgKeyEmailBuilder {
            body: Default::default(),
        }
    }
}

impl Into<GpgKeyEmail> for GpgKeyEmailBuilder {
    fn into(self) -> GpgKeyEmail {
        self.body
    }
}

/// Builder for [`GpgKeyEmail`](./struct.GpgKeyEmail.html) object.
#[derive(Debug, Clone)]
pub struct GpgKeyEmailBuilder {
    body: self::GpgKeyEmail,
}

impl GpgKeyEmailBuilder {
    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.body.email = Some(value.into());
        self
    }

    #[inline]
    pub fn verified(mut self, value: impl Into<bool>) -> Self {
        self.body.verified = Some(value.into());
        self
    }
}
