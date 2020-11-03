
/// GeneralRepoSettings contains global repository settings exposed by API
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GeneralRepoSettings {
    pub http_git_disabled: Option<bool>,
    pub mirrors_disabled: Option<bool>,
}

impl GeneralRepoSettings {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GeneralRepoSettingsBuilder {
        GeneralRepoSettingsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get_general_repository_settings() -> GeneralRepoSettingsGetBuilder {
        GeneralRepoSettingsGetBuilder
    }
}

impl Into<GeneralRepoSettings> for GeneralRepoSettingsBuilder {
    fn into(self) -> GeneralRepoSettings {
        self.body
    }
}

/// Builder for [`GeneralRepoSettings`](./struct.GeneralRepoSettings.html) object.
#[derive(Debug, Clone)]
pub struct GeneralRepoSettingsBuilder {
    body: self::GeneralRepoSettings,
}

impl GeneralRepoSettingsBuilder {
    #[inline]
    pub fn http_git_disabled(mut self, value: impl Into<bool>) -> Self {
        self.body.http_git_disabled = Some(value.into());
        self
    }

    #[inline]
    pub fn mirrors_disabled(mut self, value: impl Into<bool>) -> Self {
        self.body.mirrors_disabled = Some(value.into());
        self
    }
}

/// Builder created by [`GeneralRepoSettings::get_general_repository_settings`](./struct.GeneralRepoSettings.html#method.get_general_repository_settings) method for a `GET` operation associated with `GeneralRepoSettings`.
#[derive(Debug, Clone)]
pub struct GeneralRepoSettingsGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GeneralRepoSettingsGetBuilder {
    type Output = GeneralRepoSettings;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/settings/repository".into()
    }
}
