
/// CombinedStatus holds the combined state of several statuses for a single commit
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CombinedStatus {
    pub commit_url: Option<String>,
    pub repository: Option<Box<crate::repository::Repository>>,
    pub sha: Option<String>,
    pub state: Option<String>,
    pub statuses: Option<Vec<crate::commit_status::CommitStatus>>,
    pub total_count: Option<i64>,
    pub url: Option<String>,
}

impl CombinedStatus {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CombinedStatusBuilder {
        CombinedStatusBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_get_combined_status_by_ref() -> CombinedStatusGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingRef> {
        CombinedStatusGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_ref: core::marker::PhantomData,
        }
    }
}

impl Into<CombinedStatus> for CombinedStatusBuilder {
    fn into(self) -> CombinedStatus {
        self.body
    }
}

/// Builder for [`CombinedStatus`](./struct.CombinedStatus.html) object.
#[derive(Debug, Clone)]
pub struct CombinedStatusBuilder {
    body: self::CombinedStatus,
}

impl CombinedStatusBuilder {
    #[inline]
    pub fn commit_url(mut self, value: impl Into<String>) -> Self {
        self.body.commit_url = Some(value.into());
        self
    }

    #[inline]
    pub fn repository(mut self, value: crate::repository::Repository) -> Self {
        self.body.repository = Some(value.into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }

    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.body.state = Some(value.into());
        self
    }

    #[inline]
    pub fn statuses(mut self, value: impl Iterator<Item = crate::commit_status::CommitStatus>) -> Self {
        self.body.statuses = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn total_count(mut self, value: impl Into<i64>) -> Self {
        self.body.total_count = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`CombinedStatus::repo_get_combined_status_by_ref`](./struct.CombinedStatus.html#method.repo_get_combined_status_by_ref) method for a `GET` operation associated with `CombinedStatus`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CombinedStatusGetBuilder<Owner, Repo, Ref> {
    inner: CombinedStatusGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_ref: core::marker::PhantomData<Ref>,
}

#[derive(Debug, Default, Clone)]
struct CombinedStatusGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_ref: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo, Ref> CombinedStatusGetBuilder<Owner, Repo, Ref> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CombinedStatusGetBuilder<crate::generics::OwnerExists, Repo, Ref> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CombinedStatusGetBuilder<Owner, crate::generics::RepoExists, Ref> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of branch/tag/commit
    #[inline]
    pub fn ref_(mut self, value: impl Into<String>) -> CombinedStatusGetBuilder<Owner, Repo, crate::generics::RefExists> {
        self.inner.param_ref = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CombinedStatusGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::RefExists> {
    type Output = CombinedStatus;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/commits/{ref}/status", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), ref=self.inner.param_ref.as_ref().expect("missing parameter ref?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

impl crate::client::ResponseWrapper<CombinedStatus, CombinedStatusGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::RefExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
