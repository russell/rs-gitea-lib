#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetUsersSearchResponse {
    pub data: Option<Vec<crate::user::User>>,
    pub ok: Option<bool>,
}

impl GetUsersSearchResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GetUsersSearchResponseBuilder {
        GetUsersSearchResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_search() -> GetUsersSearchResponseGetBuilder {
        GetUsersSearchResponseGetBuilder {
            param_q: None,
            param_uid: None,
            param_page: None,
            param_limit: None,
        }
    }
}

impl Into<GetUsersSearchResponse> for GetUsersSearchResponseBuilder {
    fn into(self) -> GetUsersSearchResponse {
        self.body
    }
}

/// Builder for [`GetUsersSearchResponse`](./struct.GetUsersSearchResponse.html) object.
#[derive(Debug, Clone)]
pub struct GetUsersSearchResponseBuilder {
    body: self::GetUsersSearchResponse,
}

impl GetUsersSearchResponseBuilder {
    #[inline]
    pub fn data(mut self, value: impl Iterator<Item = crate::user::User>) -> Self {
        self.body.data = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn ok(mut self, value: impl Into<bool>) -> Self {
        self.body.ok = Some(value.into());
        self
    }
}

/// Builder created by [`GetUsersSearchResponse::user_search`](./struct.GetUsersSearchResponse.html#method.user_search) method for a `GET` operation associated with `GetUsersSearchResponse`.
#[derive(Debug, Clone)]
pub struct GetUsersSearchResponseGetBuilder {
    param_q: Option<String>,
    param_uid: Option<i64>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl GetUsersSearchResponseGetBuilder {
    /// keyword
    #[inline]
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.param_q = Some(value.into());
        self
    }

    /// ID of the user to search for
    #[inline]
    pub fn uid(mut self, value: impl Into<i64>) -> Self {
        self.param_uid = Some(value.into());
        self
    }

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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetUsersSearchResponseGetBuilder {
    type Output = GetUsersSearchResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/users/search".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("q", self.param_q.as_ref().map(std::string::ToString::to_string)),
            ("uid", self.param_uid.as_ref().map(std::string::ToString::to_string)),
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
