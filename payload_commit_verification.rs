
/// PayloadCommitVerification represents the GPG verification of a commit
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PayloadCommitVerification {
    pub payload: Option<String>,
    pub reason: Option<String>,
    pub signature: Option<String>,
    pub signer: Option<crate::payload_user::PayloadUser>,
    pub verified: Option<bool>,
}

impl PayloadCommitVerification {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PayloadCommitVerificationBuilder {
        PayloadCommitVerificationBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PayloadCommitVerification> for PayloadCommitVerificationBuilder {
    fn into(self) -> PayloadCommitVerification {
        self.body
    }
}

/// Builder for [`PayloadCommitVerification`](./struct.PayloadCommitVerification.html) object.
#[derive(Debug, Clone)]
pub struct PayloadCommitVerificationBuilder {
    body: self::PayloadCommitVerification,
}

impl PayloadCommitVerificationBuilder {
    #[inline]
    pub fn payload(mut self, value: impl Into<String>) -> Self {
        self.body.payload = Some(value.into());
        self
    }

    #[inline]
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.body.reason = Some(value.into());
        self
    }

    #[inline]
    pub fn signature(mut self, value: impl Into<String>) -> Self {
        self.body.signature = Some(value.into());
        self
    }

    #[inline]
    pub fn signer(mut self, value: crate::payload_user::PayloadUser) -> Self {
        self.body.signer = Some(value.into());
        self
    }

    #[inline]
    pub fn verified(mut self, value: impl Into<bool>) -> Self {
        self.body.verified = Some(value.into());
        self
    }
}
