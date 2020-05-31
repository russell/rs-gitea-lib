
/// SearchResults results of a successful search
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub data: Option<Vec<crate::repository::Repository>>,
    pub ok: Option<bool>,
}

impl SearchResults {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> SearchResultsBuilder {
        SearchResultsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_search() -> SearchResultsGetBuilder {
        SearchResultsGetBuilder {
            param_q: None,
            param_topic: None,
            param_include_desc: None,
            param_uid: None,
            param_priority_owner_id: None,
            param_starred_by: None,
            param_private: None,
            param_template: None,
            param_page: None,
            param_limit: None,
            param_mode: None,
            param_exclusive: None,
            param_sort: None,
            param_order: None,
        }
    }
}

impl Into<SearchResults> for SearchResultsBuilder {
    fn into(self) -> SearchResults {
        self.body
    }
}

/// Builder for [`SearchResults`](./struct.SearchResults.html) object.
#[derive(Debug, Clone)]
pub struct SearchResultsBuilder {
    body: self::SearchResults,
}

impl SearchResultsBuilder {
    #[inline]
    pub fn data(mut self, value: impl Iterator<Item = crate::repository::Repository>) -> Self {
        self.body.data = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn ok(mut self, value: impl Into<bool>) -> Self {
        self.body.ok = Some(value.into());
        self
    }
}

/// Builder created by [`SearchResults::repo_search`](./struct.SearchResults.html#method.repo_search) method for a `GET` operation associated with `SearchResults`.
#[derive(Debug, Clone)]
pub struct SearchResultsGetBuilder {
    param_q: Option<String>,
    param_topic: Option<bool>,
    param_include_desc: Option<bool>,
    param_uid: Option<i64>,
    param_priority_owner_id: Option<i64>,
    param_starred_by: Option<i64>,
    param_private: Option<bool>,
    param_template: Option<bool>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
    param_mode: Option<String>,
    param_exclusive: Option<bool>,
    param_sort: Option<String>,
    param_order: Option<String>,
}

impl SearchResultsGetBuilder {
    /// keyword
    #[inline]
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.param_q = Some(value.into());
        self
    }

    /// Limit search to repositories with keyword as topic
    #[inline]
    pub fn topic(mut self, value: impl Into<bool>) -> Self {
        self.param_topic = Some(value.into());
        self
    }

    /// include search of keyword within repository description
    #[inline]
    pub fn include_desc(mut self, value: impl Into<bool>) -> Self {
        self.param_include_desc = Some(value.into());
        self
    }

    /// search only for repos that the user with the given id owns or contributes to
    #[inline]
    pub fn uid(mut self, value: impl Into<i64>) -> Self {
        self.param_uid = Some(value.into());
        self
    }

    /// repo owner to prioritize in the results
    #[inline]
    pub fn priority_owner_id(mut self, value: impl Into<i64>) -> Self {
        self.param_priority_owner_id = Some(value.into());
        self
    }

    /// search only for repos that the user with the given id has starred
    #[inline]
    pub fn starred_by(mut self, value: impl Into<i64>) -> Self {
        self.param_starred_by = Some(value.into());
        self
    }

    /// include private repositories this user has access to (defaults to true)
    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.param_private = Some(value.into());
        self
    }

    /// include template repositories this user has access to (defaults to true)
    #[inline]
    pub fn template(mut self, value: impl Into<bool>) -> Self {
        self.param_template = Some(value.into());
        self
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// page size of results, maximum page size is 50
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.param_limit = Some(value.into());
        self
    }

    /// type of repository to search for. Supported values are "fork", "source", "mirror" and "collaborative"
    #[inline]
    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.param_mode = Some(value.into());
        self
    }

    /// if `uid` is given, search only for repos that the user owns
    #[inline]
    pub fn exclusive(mut self, value: impl Into<bool>) -> Self {
        self.param_exclusive = Some(value.into());
        self
    }

    /// sort repos by attribute. Supported values are "alpha", "created", "updated", "size", and "id". Default is "alpha"
    #[inline]
    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.param_sort = Some(value.into());
        self
    }

    /// sort order, either "asc" (ascending) or "desc" (descending). Default is "asc", ignored if "sort" is not specified.
    #[inline]
    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.param_order = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for SearchResultsGetBuilder {
    type Output = SearchResults;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/repos/search".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("q", self.param_q.as_ref().map(std::string::ToString::to_string)),
            ("topic", self.param_topic.as_ref().map(std::string::ToString::to_string)),
            ("includeDesc", self.param_include_desc.as_ref().map(std::string::ToString::to_string)),
            ("uid", self.param_uid.as_ref().map(std::string::ToString::to_string)),
            ("priority_owner_id", self.param_priority_owner_id.as_ref().map(std::string::ToString::to_string)),
            ("starredBy", self.param_starred_by.as_ref().map(std::string::ToString::to_string)),
            ("private", self.param_private.as_ref().map(std::string::ToString::to_string)),
            ("template", self.param_template.as_ref().map(std::string::ToString::to_string)),
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string)),
            ("mode", self.param_mode.as_ref().map(std::string::ToString::to_string)),
            ("exclusive", self.param_exclusive.as_ref().map(std::string::ToString::to_string)),
            ("sort", self.param_sort.as_ref().map(std::string::ToString::to_string)),
            ("order", self.param_order.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

impl crate::client::ResponseWrapper<SearchResults, SearchResultsGetBuilder> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
