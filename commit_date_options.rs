
/// CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommitDateOptions {
    pub author: Option<String>,
    pub committer: Option<String>,
}

impl CommitDateOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CommitDateOptionsBuilder {
        CommitDateOptionsBuilder {
            body: Default::default(),
        }
    }
}

impl Into<CommitDateOptions> for CommitDateOptionsBuilder {
    fn into(self) -> CommitDateOptions {
        self.body
    }
}

/// Builder for [`CommitDateOptions`](./struct.CommitDateOptions.html) object.
#[derive(Debug, Clone)]
pub struct CommitDateOptionsBuilder {
    body: self::CommitDateOptions,
}

impl CommitDateOptionsBuilder {
    #[inline]
    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.body.author = Some(value.into());
        self
    }

    #[inline]
    pub fn committer(mut self, value: impl Into<String>) -> Self {
        self.body.committer = Some(value.into());
        self
    }
}
