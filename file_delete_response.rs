
/// FileDeleteResponse contains information about a repo's file that was deleted
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileDeleteResponse {
    pub commit: Option<crate::file_commit_response::FileCommitResponse>,
    pub content: Option<crate::file_delete_response::FileDeleteResponseContent>,
    pub verification: Option<crate::payload_commit_verification::PayloadCommitVerification>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileDeleteResponseContent {}

impl FileDeleteResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> FileDeleteResponseBuilder {
        FileDeleteResponseBuilder {
            body: Default::default(),
        }
    }
}

impl Into<FileDeleteResponse> for FileDeleteResponseBuilder {
    fn into(self) -> FileDeleteResponse {
        self.body
    }
}

/// Builder for [`FileDeleteResponse`](./struct.FileDeleteResponse.html) object.
#[derive(Debug, Clone)]
pub struct FileDeleteResponseBuilder {
    body: self::FileDeleteResponse,
}

impl FileDeleteResponseBuilder {
    #[inline]
    pub fn commit(mut self, value: crate::file_commit_response::FileCommitResponse) -> Self {
        self.body.commit = Some(value.into());
        self
    }

    #[inline]
    pub fn content(mut self, value: crate::file_delete_response::FileDeleteResponseContent) -> Self {
        self.body.content = Some(value.into());
        self
    }

    #[inline]
    pub fn verification(mut self, value: crate::payload_commit_verification::PayloadCommitVerification) -> Self {
        self.body.verification = Some(value.into());
        self
    }
}

