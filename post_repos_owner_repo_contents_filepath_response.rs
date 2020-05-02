
/// FileResponse contains information about a repo's file
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostReposOwnerRepoContentsFilepathResponse {
    pub commit: Option<crate::file_commit_response::FileCommitResponse>,
    pub content: Option<crate::contents_response::ContentsResponse>,
    pub verification: Option<crate::payload_commit_verification::PayloadCommitVerification>,
}

impl PostReposOwnerRepoContentsFilepathResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PostReposOwnerRepoContentsFilepathResponseBuilder {
        PostReposOwnerRepoContentsFilepathResponseBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PostReposOwnerRepoContentsFilepathResponse> for PostReposOwnerRepoContentsFilepathResponseBuilder {
    fn into(self) -> PostReposOwnerRepoContentsFilepathResponse {
        self.body
    }
}

/// Builder for [`PostReposOwnerRepoContentsFilepathResponse`](./struct.PostReposOwnerRepoContentsFilepathResponse.html) object.
#[derive(Debug, Clone)]
pub struct PostReposOwnerRepoContentsFilepathResponseBuilder {
    body: self::PostReposOwnerRepoContentsFilepathResponse,
}

impl PostReposOwnerRepoContentsFilepathResponseBuilder {
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
