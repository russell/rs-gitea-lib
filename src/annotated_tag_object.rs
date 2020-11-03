
/// AnnotatedTagObject contains meta information of the tag object
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnnotatedTagObject {
    pub sha: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
}

impl AnnotatedTagObject {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> AnnotatedTagObjectBuilder {
        AnnotatedTagObjectBuilder {
            body: Default::default(),
        }
    }
}

impl Into<AnnotatedTagObject> for AnnotatedTagObjectBuilder {
    fn into(self) -> AnnotatedTagObject {
        self.body
    }
}

/// Builder for [`AnnotatedTagObject`](./struct.AnnotatedTagObject.html) object.
#[derive(Debug, Clone)]
pub struct AnnotatedTagObjectBuilder {
    body: self::AnnotatedTagObject,
}

impl AnnotatedTagObjectBuilder {
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
