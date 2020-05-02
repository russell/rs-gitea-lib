
/// GitEntry represents a git tree
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitEntry {
    pub mode: Option<String>,
    pub path: Option<String>,
    pub sha: Option<String>,
    pub size: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
}

impl GitEntry {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GitEntryBuilder {
        GitEntryBuilder {
            body: Default::default(),
        }
    }
}

impl Into<GitEntry> for GitEntryBuilder {
    fn into(self) -> GitEntry {
        self.body
    }
}

/// Builder for [`GitEntry`](./struct.GitEntry.html) object.
#[derive(Debug, Clone)]
pub struct GitEntryBuilder {
    body: self::GitEntry,
}

impl GitEntryBuilder {
    #[inline]
    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.body.mode = Some(value.into());
        self
    }

    #[inline]
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.body.path = Some(value.into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }

    #[inline]
    pub fn size(mut self, value: impl Into<i64>) -> Self {
        self.body.size = Some(value.into());
        self
    }

    #[inline]
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.body.type_ = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}
