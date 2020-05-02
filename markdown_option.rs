
/// MarkdownOption markdown options
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MarkdownOption {
    /// Context to render
    ///
    /// in: body
    #[serde(rename = "Context")]
    pub context: Option<String>,
    /// Mode to render
    ///
    /// in: body
    #[serde(rename = "Mode")]
    pub mode: Option<String>,
    /// Text markdown to render
    ///
    /// in: body
    #[serde(rename = "Text")]
    pub text: Option<String>,
    /// Is it a wiki page ?
    ///
    /// in: body
    #[serde(rename = "Wiki")]
    pub wiki: Option<bool>,
}

impl MarkdownOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> MarkdownOptionBuilder {
        MarkdownOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn render_markdown() -> MarkdownOptionPostBuilder {
        MarkdownOptionPostBuilder {
            body: Default::default(),
        }
    }
}

impl Into<MarkdownOption> for MarkdownOptionBuilder {
    fn into(self) -> MarkdownOption {
        self.body
    }
}

impl Into<MarkdownOption> for MarkdownOptionPostBuilder {
    fn into(self) -> MarkdownOption {
        self.body
    }
}

/// Builder for [`MarkdownOption`](./struct.MarkdownOption.html) object.
#[derive(Debug, Clone)]
pub struct MarkdownOptionBuilder {
    body: self::MarkdownOption,
}

impl MarkdownOptionBuilder {
    /// Context to render
    ///
    /// in: body
    #[inline]
    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.body.context = Some(value.into());
        self
    }

    /// Mode to render
    ///
    /// in: body
    #[inline]
    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.body.mode = Some(value.into());
        self
    }

    /// Text markdown to render
    ///
    /// in: body
    #[inline]
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.body.text = Some(value.into());
        self
    }

    /// Is it a wiki page ?
    ///
    /// in: body
    #[inline]
    pub fn wiki(mut self, value: impl Into<bool>) -> Self {
        self.body.wiki = Some(value.into());
        self
    }
}

/// Builder created by [`MarkdownOption::render_markdown`](./struct.MarkdownOption.html#method.render_markdown) method for a `POST` operation associated with `MarkdownOption`.
#[derive(Debug, Clone)]
pub struct MarkdownOptionPostBuilder {
    body: self::MarkdownOption,
}

impl MarkdownOptionPostBuilder {
    /// Context to render
    ///
    /// in: body
    #[inline]
    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.body.context = Some(value.into());
        self
    }

    /// Mode to render
    ///
    /// in: body
    #[inline]
    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.body.mode = Some(value.into());
        self
    }

    /// Text markdown to render
    ///
    /// in: body
    #[inline]
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.body.text = Some(value.into());
        self
    }

    /// Is it a wiki page ?
    ///
    /// in: body
    #[inline]
    pub fn wiki(mut self, value: impl Into<bool>) -> Self {
        self.body.wiki = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MarkdownOptionPostBuilder {
    type Output = String;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/markdown".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}
