
/// GeneralUISettings contains global ui settings exposed by API
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GeneralUiSettings {
    pub allowed_reactions: Option<Vec<String>>,
}

impl GeneralUiSettings {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GeneralUiSettingsBuilder {
        GeneralUiSettingsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get_general_ui_settings() -> GeneralUiSettingsGetBuilder {
        GeneralUiSettingsGetBuilder
    }
}

impl Into<GeneralUiSettings> for GeneralUiSettingsBuilder {
    fn into(self) -> GeneralUiSettings {
        self.body
    }
}

/// Builder for [`GeneralUiSettings`](./struct.GeneralUiSettings.html) object.
#[derive(Debug, Clone)]
pub struct GeneralUiSettingsBuilder {
    body: self::GeneralUiSettings,
}

impl GeneralUiSettingsBuilder {
    #[inline]
    pub fn allowed_reactions(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.allowed_reactions = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`GeneralUiSettings::get_general_ui_settings`](./struct.GeneralUiSettings.html#method.get_general_ui_settings) method for a `GET` operation associated with `GeneralUiSettings`.
#[derive(Debug, Clone)]
pub struct GeneralUiSettingsGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GeneralUiSettingsGetBuilder {
    type Output = GeneralUiSettings;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/settings/ui".into()
    }
}
