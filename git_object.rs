#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitObject {
    pub sha: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
}

impl GitObject {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GitObjectBuilder {
        GitObjectBuilder {
            body: Default::default(),
        }
    }
}

impl Into<GitObject> for GitObjectBuilder {
    fn into(self) -> GitObject {
        self.body
    }
}

/// Builder for [`GitObject`](./struct.GitObject.html) object.
#[derive(Debug, Clone)]
pub struct GitObjectBuilder {
    body: self::GitObject,
}

impl GitObjectBuilder {
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }

    #[inline]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.body.type_ = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}
