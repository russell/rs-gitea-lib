
/// StopWatch represent a running stopwatch
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StopWatch {
    pub created: Option<String>,
    pub issue_index: Option<i64>,
    pub issue_title: Option<String>,
    pub repo_name: Option<String>,
    pub repo_owner_name: Option<String>,
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
        StopWatchGetBuilder {
            param_page: None,
            param_limit: None,
        }
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

    #[inline]
    pub fn issue_title(mut self, value: impl Into<String>) -> Self {
        self.body.issue_title = Some(value.into());
        self
    }

    #[inline]
    pub fn repo_name(mut self, value: impl Into<String>) -> Self {
        self.body.repo_name = Some(value.into());
        self
    }

    #[inline]
    pub fn repo_owner_name(mut self, value: impl Into<String>) -> Self {
        self.body.repo_owner_name = Some(value.into());
        self
    }
}

/// Builder created by [`StopWatch::user_get_stop_watches`](./struct.StopWatch.html#method.user_get_stop_watches) method for a `GET` operation associated with `StopWatch`.
#[derive(Debug, Clone)]
pub struct StopWatchGetBuilder {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl StopWatchGetBuilder {
    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for StopWatchGetBuilder {
    type Output = Vec<StopWatch>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/stopwatches".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
