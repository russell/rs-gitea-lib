#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RepoCommit {
    pub author: Option<crate::commit_user::CommitUser>,
    pub committer: Option<crate::commit_user::CommitUser>,
    pub message: Option<String>,
    pub tree: Option<crate::commit_meta::CommitMeta>,
    pub url: Option<String>,
}

impl RepoCommit {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> RepoCommitBuilder {
        RepoCommitBuilder {
            body: Default::default(),
        }
    }
}

impl Into<RepoCommit> for RepoCommitBuilder {
    fn into(self) -> RepoCommit {
        self.body
    }
}

/// Builder for [`RepoCommit`](./struct.RepoCommit.html) object.
#[derive(Debug, Clone)]
pub struct RepoCommitBuilder {
    body: self::RepoCommit,
}

impl RepoCommitBuilder {
    #[inline]
    pub fn author(mut self, value: crate::commit_user::CommitUser) -> Self {
        self.body.author = Some(value.into());
        self
    }

    #[inline]
    pub fn committer(mut self, value: crate::commit_user::CommitUser) -> Self {
        self.body.committer = Some(value.into());
        self
    }

    #[inline]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.body.message = Some(value.into());
        self
    }

    #[inline]
    pub fn tree(mut self, value: crate::commit_meta::CommitMeta) -> Self {
        self.body.tree = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}
