
/// GeneralAPISettings contains global api settings exposed by it
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GeneralApiSettings {
    pub default_git_trees_per_page: Option<i64>,
    pub default_max_blob_size: Option<i64>,
    pub default_paging_num: Option<i64>,
    pub max_response_items: Option<i64>,
}

impl GeneralApiSettings {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GeneralApiSettingsBuilder {
        GeneralApiSettingsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get_general_api_settings() -> GeneralApiSettingsGetBuilder {
        GeneralApiSettingsGetBuilder
    }
}

impl Into<GeneralApiSettings> for GeneralApiSettingsBuilder {
    fn into(self) -> GeneralApiSettings {
        self.body
    }
}

/// Builder for [`GeneralApiSettings`](./struct.GeneralApiSettings.html) object.
#[derive(Debug, Clone)]
pub struct GeneralApiSettingsBuilder {
    body: self::GeneralApiSettings,
}

impl GeneralApiSettingsBuilder {
    #[inline]
    pub fn default_git_trees_per_page(mut self, value: impl Into<i64>) -> Self {
        self.body.default_git_trees_per_page = Some(value.into());
        self
    }

    #[inline]
    pub fn default_max_blob_size(mut self, value: impl Into<i64>) -> Self {
        self.body.default_max_blob_size = Some(value.into());
        self
    }

    #[inline]
    pub fn default_paging_num(mut self, value: impl Into<i64>) -> Self {
        self.body.default_paging_num = Some(value.into());
        self
    }

    #[inline]
    pub fn max_response_items(mut self, value: impl Into<i64>) -> Self {
        self.body.max_response_items = Some(value.into());
        self
    }
}

/// Builder created by [`GeneralApiSettings::get_general_api_settings`](./struct.GeneralApiSettings.html#method.get_general_api_settings) method for a `GET` operation associated with `GeneralApiSettings`.
#[derive(Debug, Clone)]
pub struct GeneralApiSettingsGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GeneralApiSettingsGetBuilder {
    type Output = GeneralApiSettings;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/settings/api".into()
    }
}
