
/// UserHeatmapData represents the data needed to create a heatmap
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserHeatmapData {
    pub contributions: Option<i64>,
    pub timestamp: Option<i64>,
}

impl UserHeatmapData {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> UserHeatmapDataBuilder {
        UserHeatmapDataBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_get_heatmap_data() -> UserHeatmapDataGetBuilder<crate::generics::MissingUsername> {
        UserHeatmapDataGetBuilder {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }
}

impl Into<UserHeatmapData> for UserHeatmapDataBuilder {
    fn into(self) -> UserHeatmapData {
        self.body
    }
}

/// Builder for [`UserHeatmapData`](./struct.UserHeatmapData.html) object.
#[derive(Debug, Clone)]
pub struct UserHeatmapDataBuilder {
    body: self::UserHeatmapData,
}

impl UserHeatmapDataBuilder {
    #[inline]
    pub fn contributions(mut self, value: impl Into<i64>) -> Self {
        self.body.contributions = Some(value.into());
        self
    }

    #[inline]
    pub fn timestamp(mut self, value: impl Into<i64>) -> Self {
        self.body.timestamp = Some(value.into());
        self
    }
}

/// Builder created by [`UserHeatmapData::user_get_heatmap_data`](./struct.UserHeatmapData.html#method.user_get_heatmap_data) method for a `GET` operation associated with `UserHeatmapData`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserHeatmapDataGetBuilder<Username> {
    inner: UserHeatmapDataGetBuilderContainer,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct UserHeatmapDataGetBuilderContainer {
    param_username: Option<String>,
}

impl<Username> UserHeatmapDataGetBuilder<Username> {
    /// username of user to get
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> UserHeatmapDataGetBuilder<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserHeatmapDataGetBuilder<crate::generics::UsernameExists> {
    type Output = Vec<UserHeatmapData>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/heatmap", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }
}
