
/// PayloadCommit represents a commit
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PayloadCommit {
    pub added: Option<Vec<String>>,
    pub author: Option<crate::payload_user::PayloadUser>,
    pub committer: Option<crate::payload_user::PayloadUser>,
    /// sha1 hash of the commit
    pub id: Option<String>,
    pub message: Option<String>,
    pub modified: Option<Vec<String>>,
    pub removed: Option<Vec<String>>,
    pub timestamp: Option<String>,
    pub url: Option<String>,
    pub verification: Option<crate::payload_commit_verification::PayloadCommitVerification>,
}

impl PayloadCommit {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PayloadCommitBuilder {
        PayloadCommitBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PayloadCommit> for PayloadCommitBuilder {
    fn into(self) -> PayloadCommit {
        self.body
    }
}

/// Builder for [`PayloadCommit`](./struct.PayloadCommit.html) object.
#[derive(Debug, Clone)]
pub struct PayloadCommitBuilder {
    body: self::PayloadCommit,
}

impl PayloadCommitBuilder {
    #[inline]
    pub fn added(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.added = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn author(mut self, value: crate::payload_user::PayloadUser) -> Self {
        self.body.author = Some(value.into());
        self
    }

    #[inline]
    pub fn committer(mut self, value: crate::payload_user::PayloadUser) -> Self {
        self.body.committer = Some(value.into());
        self
    }

    /// sha1 hash of the commit
    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.body.message = Some(value.into());
        self
    }

    #[inline]
    pub fn modified(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.modified = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn removed(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.removed = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.body.timestamp = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }

    #[inline]
    pub fn verification(mut self, value: crate::payload_commit_verification::PayloadCommitVerification) -> Self {
        self.body.verification = Some(value.into());
        self
    }
}
