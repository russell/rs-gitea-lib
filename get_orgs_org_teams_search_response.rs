#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetOrgsOrgTeamsSearchResponse {
    pub data: Option<Vec<crate::team::Team>>,
    pub ok: Option<bool>,
}

impl GetOrgsOrgTeamsSearchResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GetOrgsOrgTeamsSearchResponseBuilder {
        GetOrgsOrgTeamsSearchResponseBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn team_search() -> GetOrgsOrgTeamsSearchResponseGetBuilder<crate::generics::MissingOrg> {
        GetOrgsOrgTeamsSearchResponseGetBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }
}

impl Into<GetOrgsOrgTeamsSearchResponse> for GetOrgsOrgTeamsSearchResponseBuilder {
    fn into(self) -> GetOrgsOrgTeamsSearchResponse {
        self.body
    }
}

/// Builder for [`GetOrgsOrgTeamsSearchResponse`](./struct.GetOrgsOrgTeamsSearchResponse.html) object.
#[derive(Debug, Clone)]
pub struct GetOrgsOrgTeamsSearchResponseBuilder {
    body: self::GetOrgsOrgTeamsSearchResponse,
}

impl GetOrgsOrgTeamsSearchResponseBuilder {
    #[inline]
    pub fn data(mut self, value: impl Iterator<Item = crate::team::Team>) -> Self {
        self.body.data = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn ok(mut self, value: impl Into<bool>) -> Self {
        self.body.ok = Some(value.into());
        self
    }
}

/// Builder created by [`GetOrgsOrgTeamsSearchResponse::team_search`](./struct.GetOrgsOrgTeamsSearchResponse.html#method.team_search) method for a `GET` operation associated with `GetOrgsOrgTeamsSearchResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GetOrgsOrgTeamsSearchResponseGetBuilder<Org> {
    inner: GetOrgsOrgTeamsSearchResponseGetBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct GetOrgsOrgTeamsSearchResponseGetBuilderContainer {
    param_org: Option<String>,
    param_q: Option<String>,
    param_include_desc: Option<bool>,
    param_limit: Option<i64>,
    param_page: Option<i64>,
}

impl<Org> GetOrgsOrgTeamsSearchResponseGetBuilder<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> GetOrgsOrgTeamsSearchResponseGetBuilder<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// keywords to search
    #[inline]
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.inner.param_q = Some(value.into());
        self
    }

    /// include search within team description (defaults to true)
    #[inline]
    pub fn include_desc(mut self, value: impl Into<bool>) -> Self {
        self.inner.param_include_desc = Some(value.into());
        self
    }

    /// limit size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_limit = Some(value.into());
        self
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetOrgsOrgTeamsSearchResponseGetBuilder<crate::generics::OrgExists> {
    type Output = GetOrgsOrgTeamsSearchResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/teams/search", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("q", self.inner.param_q.as_ref().map(std::string::ToString::to_string)),
            ("include_desc", self.inner.param_include_desc.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
