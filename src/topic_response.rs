
/// TopicResponse for returning topics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TopicResponse {
    pub created: Option<String>,
    pub id: Option<i64>,
    pub repo_count: Option<i64>,
    pub topic_name: Option<String>,
    pub updated: Option<String>,
}

impl TopicResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TopicResponseBuilder {
        TopicResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn topic_search() -> TopicResponseGetBuilder<crate::generics::MissingQ> {
        TopicResponseGetBuilder {
            inner: Default::default(),
            _param_q: core::marker::PhantomData,
        }
    }
}

impl Into<TopicResponse> for TopicResponseBuilder {
    fn into(self) -> TopicResponse {
        self.body
    }
}

/// Builder for [`TopicResponse`](./struct.TopicResponse.html) object.
#[derive(Debug, Clone)]
pub struct TopicResponseBuilder {
    body: self::TopicResponse,
}

impl TopicResponseBuilder {
    #[inline]
    pub fn created(mut self, value: impl Into<String>) -> Self {
        self.body.created = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn repo_count(mut self, value: impl Into<i64>) -> Self {
        self.body.repo_count = Some(value.into());
        self
    }

    #[inline]
    pub fn topic_name(mut self, value: impl Into<String>) -> Self {
        self.body.topic_name = Some(value.into());
        self
    }

    #[inline]
    pub fn updated(mut self, value: impl Into<String>) -> Self {
        self.body.updated = Some(value.into());
        self
    }
}

/// Builder created by [`TopicResponse::topic_search`](./struct.TopicResponse.html#method.topic_search) method for a `GET` operation associated with `TopicResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TopicResponseGetBuilder<Q> {
    inner: TopicResponseGetBuilderContainer,
    _param_q: core::marker::PhantomData<Q>,
}

#[derive(Debug, Default, Clone)]
struct TopicResponseGetBuilderContainer {
    param_q: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Q> TopicResponseGetBuilder<Q> {
    /// keywords to search
    #[inline]
    pub fn q(mut self, value: impl Into<String>) -> TopicResponseGetBuilder<crate::generics::QExists> {
        self.inner.param_q = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TopicResponseGetBuilder<crate::generics::QExists> {
    type Output = Vec<TopicResponse>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/topics/search".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("q", self.inner.param_q.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

impl crate::client::ResponseWrapper<Vec<TopicResponse>, TopicResponseGetBuilder<crate::generics::QExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
