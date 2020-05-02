
/// ExternalWiki represents setting for external wiki
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExternalWiki {
    /// URL of external wiki.
    pub external_wiki_url: Option<String>,
}

impl ExternalWiki {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ExternalWikiBuilder {
        ExternalWikiBuilder {
            body: Default::default(),
        }
    }
}

impl Into<ExternalWiki> for ExternalWikiBuilder {
    fn into(self) -> ExternalWiki {
        self.body
    }
}

/// Builder for [`ExternalWiki`](./struct.ExternalWiki.html) object.
#[derive(Debug, Clone)]
pub struct ExternalWikiBuilder {
    body: self::ExternalWiki,
}

impl ExternalWikiBuilder {
    /// URL of external wiki.
    #[inline]
    pub fn external_wiki_url(mut self, value: impl Into<String>) -> Self {
        self.body.external_wiki_url = Some(value.into());
        self
    }
}
