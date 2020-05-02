
/// PullRequestMeta PR info if an issue is a PR
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PullRequestMeta {
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
}

impl PullRequestMeta {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PullRequestMetaBuilder {
        PullRequestMetaBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PullRequestMeta> for PullRequestMetaBuilder {
    fn into(self) -> PullRequestMeta {
        self.body
    }
}

/// Builder for [`PullRequestMeta`](./struct.PullRequestMeta.html) object.
#[derive(Debug, Clone)]
pub struct PullRequestMetaBuilder {
    body: self::PullRequestMeta,
}

impl PullRequestMetaBuilder {
    #[inline]
    pub fn merged(mut self, value: impl Into<bool>) -> Self {
        self.body.merged = Some(value.into());
        self
    }

    #[inline]
    pub fn merged_at(mut self, value: impl Into<String>) -> Self {
        self.body.merged_at = Some(value.into());
        self
    }
}
