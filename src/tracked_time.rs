
/// TrackedTime worked time for an issue / pr
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TrackedTime {
    pub created: Option<String>,
    pub id: Option<i64>,
    pub issue: Option<crate::issue::Issue>,
    /// deprecated (only for backwards compatibility)
    pub issue_id: Option<i64>,
    /// Time in seconds
    pub time: Option<i64>,
    /// deprecated (only for backwards compatibility)
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
}

impl TrackedTime {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TrackedTimeBuilder {
        TrackedTimeBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_tracked_times() -> TrackedTimeGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        TrackedTimeGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_tracked_times() -> TrackedTimeGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        TrackedTimeGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[deprecated]
    #[inline]
    pub fn user_tracked_times() -> TrackedTimeGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingUser> {
        TrackedTimeGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_user: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_current_tracked_times() -> TrackedTimeGetBuilder3 {
        TrackedTimeGetBuilder3 {
            param_since: None,
            param_before: None,
        }
    }
}

impl Into<TrackedTime> for TrackedTimeBuilder {
    fn into(self) -> TrackedTime {
        self.body
    }
}

/// Builder for [`TrackedTime`](./struct.TrackedTime.html) object.
#[derive(Debug, Clone)]
pub struct TrackedTimeBuilder {
    body: self::TrackedTime,
}

impl TrackedTimeBuilder {
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
    pub fn issue(mut self, value: crate::issue::Issue) -> Self {
        self.body.issue = Some(value.into());
        self
    }

    /// deprecated (only for backwards compatibility)
    #[inline]
    pub fn issue_id(mut self, value: impl Into<i64>) -> Self {
        self.body.issue_id = Some(value.into());
        self
    }

    /// Time in seconds
    #[inline]
    pub fn time(mut self, value: impl Into<i64>) -> Self {
        self.body.time = Some(value.into());
        self
    }

    /// deprecated (only for backwards compatibility)
    #[inline]
    pub fn user_id(mut self, value: impl Into<i64>) -> Self {
        self.body.user_id = Some(value.into());
        self
    }

    #[inline]
    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.body.user_name = Some(value.into());
        self
    }
}

/// Builder created by [`TrackedTime::issue_tracked_times`](./struct.TrackedTime.html#method.issue_tracked_times) method for a `GET` operation associated with `TrackedTime`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TrackedTimeGetBuilder<Owner, Repo, Index> {
    inner: TrackedTimeGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct TrackedTimeGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_user: Option<String>,
    param_since: Option<String>,
    param_before: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo, Index> TrackedTimeGetBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> TrackedTimeGetBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// optional filter by user (available for issue managers)
    #[inline]
    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.inner.param_user = Some(value.into());
        self
    }

    /// Only show times updated after the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.inner.param_since = Some(value.into());
        self
    }

    /// Only show times updated before the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.inner.param_before = Some(value.into());
        self
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TrackedTimeGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<TrackedTime>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/times", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("user", self.inner.param_user.as_ref().map(std::string::ToString::to_string)),
            ("since", self.inner.param_since.as_ref().map(std::string::ToString::to_string)),
            ("before", self.inner.param_before.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`TrackedTime::repo_tracked_times`](./struct.TrackedTime.html#method.repo_tracked_times) method for a `GET` operation associated with `TrackedTime`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TrackedTimeGetBuilder1<Owner, Repo> {
    inner: TrackedTimeGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct TrackedTimeGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_user: Option<String>,
    param_since: Option<String>,
    param_before: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> TrackedTimeGetBuilder1<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder1<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder1<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// optional filter by user (available for issue managers)
    #[inline]
    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.inner.param_user = Some(value.into());
        self
    }

    /// Only show times updated after the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.inner.param_since = Some(value.into());
        self
    }

    /// Only show times updated before the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.inner.param_before = Some(value.into());
        self
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TrackedTimeGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<TrackedTime>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/times", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("user", self.inner.param_user.as_ref().map(std::string::ToString::to_string)),
            ("since", self.inner.param_since.as_ref().map(std::string::ToString::to_string)),
            ("before", self.inner.param_before.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

impl crate::client::ResponseWrapper<Vec<TrackedTime>, TrackedTimeGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`TrackedTime::user_tracked_times`](./struct.TrackedTime.html#method.user_tracked_times) method for a `GET` operation associated with `TrackedTime`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TrackedTimeGetBuilder2<Owner, Repo, User> {
    inner: TrackedTimeGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_user: core::marker::PhantomData<User>,
}

#[derive(Debug, Default, Clone)]
struct TrackedTimeGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_user: Option<String>,
}

impl<Owner, Repo, User> TrackedTimeGetBuilder2<Owner, Repo, User> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder2<crate::generics::OwnerExists, Repo, User> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder2<Owner, crate::generics::RepoExists, User> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// username of user
    #[inline]
    pub fn user(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder2<Owner, Repo, crate::generics::UserExists> {
        self.inner.param_user = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TrackedTimeGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::UserExists> {
    type Output = Vec<TrackedTime>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/times/{user}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), user=self.inner.param_user.as_ref().expect("missing parameter user?")).into()
    }
}

impl crate::client::ResponseWrapper<Vec<TrackedTime>, TrackedTimeGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::UserExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`TrackedTime::user_current_tracked_times`](./struct.TrackedTime.html#method.user_current_tracked_times) method for a `GET` operation associated with `TrackedTime`.
#[derive(Debug, Clone)]
pub struct TrackedTimeGetBuilder3 {
    param_since: Option<String>,
    param_before: Option<String>,
}

impl TrackedTimeGetBuilder3 {
    /// Only show times updated after the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.param_since = Some(value.into());
        self
    }

    /// Only show times updated before the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.param_before = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TrackedTimeGetBuilder3 {
    type Output = Vec<TrackedTime>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/times".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("since", self.param_since.as_ref().map(std::string::ToString::to_string)),
            ("before", self.param_before.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
