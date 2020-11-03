#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileCommitResponse {
    pub author: Option<crate::commit_user::CommitUser>,
    pub committer: Option<crate::commit_user::CommitUser>,
    pub created: Option<String>,
    pub html_url: Option<String>,
    pub message: Option<String>,
    pub parents: Option<Vec<crate::commit_meta::CommitMeta>>,
    pub sha: Option<String>,
    pub tree: Option<crate::commit_meta::CommitMeta>,
    pub url: Option<String>,
}

impl FileCommitResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> FileCommitResponseBuilder {
        FileCommitResponseBuilder {
            body: Default::default(),
        }
    }
}

impl Into<FileCommitResponse> for FileCommitResponseBuilder {
    fn into(self) -> FileCommitResponse {
        self.body
    }
}

/// Builder for [`FileCommitResponse`](./struct.FileCommitResponse.html) object.
#[derive(Debug, Clone)]
pub struct FileCommitResponseBuilder {
    body: self::FileCommitResponse,
}

impl FileCommitResponseBuilder {
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
    pub fn created(mut self, value: impl Into<String>) -> Self {
        self.body.created = Some(value.into());
        self
    }

    #[inline]
    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.body.html_url = Some(value.into());
        self
    }

    #[inline]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.body.message = Some(value.into());
        self
    }

    #[inline]
    pub fn parents(mut self, value: impl Iterator<Item = crate::commit_meta::CommitMeta>) -> Self {
        self.body.parents = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
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
