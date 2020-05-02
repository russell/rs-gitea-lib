
/// APIError is an api error with a message
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub message: Option<String>,
    pub url: Option<String>,
}

impl ApiError {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ApiErrorBuilder {
        ApiErrorBuilder {
            body: Default::default(),
        }
    }
}

impl Into<ApiError> for ApiErrorBuilder {
    fn into(self) -> ApiError {
        self.body
    }
}

/// Builder for [`ApiError`](./struct.ApiError.html) object.
#[derive(Debug, Clone)]
pub struct ApiErrorBuilder {
    body: self::ApiError,
}

impl ApiErrorBuilder {
    #[inline]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.body.message = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}
