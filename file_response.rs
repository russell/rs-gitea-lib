
/// FileResponse contains information about a repo's file
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileResponse {
    pub commit: Option<crate::file_commit_response::FileCommitResponse>,
    pub content: Option<crate::contents_response::ContentsResponse>,
    pub verification: Option<crate::payload_commit_verification::PayloadCommitVerification>,
}

impl FileResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> FileResponseBuilder {
        FileResponseBuilder {
            body: Default::default(),
        }
    }
}

impl Into<FileResponse> for FileResponseBuilder {
    fn into(self) -> FileResponse {
        self.body
    }
}

/// Builder for [`FileResponse`](./struct.FileResponse.html) object.
#[derive(Debug, Clone)]
pub struct FileResponseBuilder {
    body: self::FileResponse,
}

impl FileResponseBuilder {
    #[inline]
    pub fn commit(mut self, value: crate::file_commit_response::FileCommitResponse) -> Self {
        self.body.commit = Some(value.into());
        self
    }

    #[inline]
    pub fn content(mut self, value: crate::contents_response::ContentsResponse) -> Self {
        self.body.content = Some(value.into());
        self
    }

    #[inline]
    pub fn verification(mut self, value: crate::payload_commit_verification::PayloadCommitVerification) -> Self {
        self.body.verification = Some(value.into());
        self
    }
}
