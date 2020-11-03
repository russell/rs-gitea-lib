
/// Cron represents a Cron task
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Cron {
    pub exec_times: Option<i64>,
    pub name: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub schedule: Option<String>,
}

impl Cron {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CronBuilder {
        CronBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn admin_cron_list() -> CronGetBuilder {
        CronGetBuilder {
            param_page: None,
            param_limit: None,
        }
    }
}

impl Into<Cron> for CronBuilder {
    fn into(self) -> Cron {
        self.body
    }
}

/// Builder for [`Cron`](./struct.Cron.html) object.
#[derive(Debug, Clone)]
pub struct CronBuilder {
    body: self::Cron,
}

impl CronBuilder {
    #[inline]
    pub fn exec_times(mut self, value: impl Into<i64>) -> Self {
        self.body.exec_times = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.body.next = Some(value.into());
        self
    }

    #[inline]
    pub fn prev(mut self, value: impl Into<String>) -> Self {
        self.body.prev = Some(value.into());
        self
    }

    #[inline]
    pub fn schedule(mut self, value: impl Into<String>) -> Self {
        self.body.schedule = Some(value.into());
        self
    }
}

/// Builder created by [`Cron::admin_cron_list`](./struct.Cron.html#method.admin_cron_list) method for a `GET` operation associated with `Cron`.
#[derive(Debug, Clone)]
pub struct CronGetBuilder {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl CronGetBuilder {
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CronGetBuilder {
    type Output = Vec<Cron>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/admin/cron".into()
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

impl crate::client::ResponseWrapper<Vec<Cron>, CronGetBuilder> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
