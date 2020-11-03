#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommitMeta {
    pub created: Option<String>,
    pub sha: Option<String>,
    pub url: Option<String>,
}

impl CommitMeta {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CommitMetaBuilder {
        CommitMetaBuilder {
            body: Default::default(),
        }
    }
}

impl Into<CommitMeta> for CommitMetaBuilder {
    fn into(self) -> CommitMeta {
        self.body
    }
}

/// Builder for [`CommitMeta`](./struct.CommitMeta.html) object.
#[derive(Debug, Clone)]
pub struct CommitMetaBuilder {
    body: self::CommitMeta,
}

impl CommitMetaBuilder {
    #[inline]
    pub fn created(mut self, value: impl Into<String>) -> Self {
        self.body.created = Some(value.into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}
