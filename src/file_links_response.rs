
/// FileLinksResponse contains the links for a repo's file
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileLinksResponse {
    pub git: Option<String>,
    pub html: Option<String>,
    #[serde(rename = "self")]
    pub self_: Option<String>,
}

impl FileLinksResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> FileLinksResponseBuilder {
        FileLinksResponseBuilder {
            body: Default::default(),
        }
    }
}

impl Into<FileLinksResponse> for FileLinksResponseBuilder {
    fn into(self) -> FileLinksResponse {
        self.body
    }
}

/// Builder for [`FileLinksResponse`](./struct.FileLinksResponse.html) object.
#[derive(Debug, Clone)]
pub struct FileLinksResponseBuilder {
    body: self::FileLinksResponse,
}

impl FileLinksResponseBuilder {
    #[inline]
    pub fn git(mut self, value: impl Into<String>) -> Self {
        self.body.git = Some(value.into());
        self
    }

    #[inline]
    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.body.html = Some(value.into());
        self
    }

    #[inline]
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.body.self_ = Some(value.into());
        self
    }
}
