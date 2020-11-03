
/// GeneralAttachmentSettings contains global Attachment settings exposed by API
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GeneralAttachmentSettings {
    pub allowed_types: Option<String>,
    pub enabled: Option<bool>,
    pub max_files: Option<i64>,
    pub max_size: Option<i64>,
}

impl GeneralAttachmentSettings {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GeneralAttachmentSettingsBuilder {
        GeneralAttachmentSettingsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get_general_attachment_settings() -> GeneralAttachmentSettingsGetBuilder {
        GeneralAttachmentSettingsGetBuilder
    }
}

impl Into<GeneralAttachmentSettings> for GeneralAttachmentSettingsBuilder {
    fn into(self) -> GeneralAttachmentSettings {
        self.body
    }
}

/// Builder for [`GeneralAttachmentSettings`](./struct.GeneralAttachmentSettings.html) object.
#[derive(Debug, Clone)]
pub struct GeneralAttachmentSettingsBuilder {
    body: self::GeneralAttachmentSettings,
}

impl GeneralAttachmentSettingsBuilder {
    #[inline]
    pub fn allowed_types(mut self, value: impl Into<String>) -> Self {
        self.body.allowed_types = Some(value.into());
        self
    }

    #[inline]
    pub fn enabled(mut self, value: impl Into<bool>) -> Self {
        self.body.enabled = Some(value.into());
        self
    }

    #[inline]
    pub fn max_files(mut self, value: impl Into<i64>) -> Self {
        self.body.max_files = Some(value.into());
        self
    }

    #[inline]
    pub fn max_size(mut self, value: impl Into<i64>) -> Self {
        self.body.max_size = Some(value.into());
        self
    }
}

/// Builder created by [`GeneralAttachmentSettings::get_general_attachment_settings`](./struct.GeneralAttachmentSettings.html#method.get_general_attachment_settings) method for a `GET` operation associated with `GeneralAttachmentSettings`.
#[derive(Debug, Clone)]
pub struct GeneralAttachmentSettingsGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GeneralAttachmentSettingsGetBuilder {
    type Output = GeneralAttachmentSettings;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/settings/attachment".into()
    }
}
