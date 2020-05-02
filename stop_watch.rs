
/// StopWatch represent a running stopwatch
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StopWatch {
    pub created: Option<String>,
    pub issue_index: Option<i64>,
}

impl StopWatch {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> StopWatchBuilder {
        StopWatchBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_get_stop_watches() -> StopWatchGetBuilder {
        StopWatchGetBuilder
    }
}

impl Into<StopWatch> for StopWatchBuilder {
    fn into(self) -> StopWatch {
        self.body
    }
}

/// Builder for [`StopWatch`](./struct.StopWatch.html) object.
#[derive(Debug, Clone)]
pub struct StopWatchBuilder {
    body: self::StopWatch,
}

impl StopWatchBuilder {
    #[inline]
    pub fn created(mut self, value: impl Into<String>) -> Self {
        self.body.created = Some(value.into());
        self
    }

    #[inline]
    pub fn issue_index(mut self, value: impl Into<i64>) -> Self {
        self.body.issue_index = Some(value.into());
        self
    }
}

/// Builder created by [`StopWatch::user_get_stop_watches`](./struct.StopWatch.html#method.user_get_stop_watches) method for a `GET` operation associated with `StopWatch`.
#[derive(Debug, Clone)]
pub struct StopWatchGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for StopWatchGetBuilder {
    type Output = Vec<StopWatch>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/stopwatches".into()
    }
}
