
/// NotificationThread expose Notification on API
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotificationThread {
    pub id: Option<i64>,
    pub pinned: Option<bool>,
    pub repository: Option<Box<crate::repository::Repository>>,
    pub subject: Option<crate::notification_subject::NotificationSubject>,
    pub unread: Option<bool>,
    pub updated_at: Option<String>,
    pub url: Option<String>,
}

impl NotificationThread {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> NotificationThreadBuilder {
        NotificationThreadBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn notify_get_list() -> NotificationThreadGetBuilder {
        NotificationThreadGetBuilder {
            param_all: None,
            param_status_types: None,
            param_since: None,
            param_before: None,
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn notify_get_thread() -> NotificationThreadGetBuilder1<crate::generics::MissingId> {
        NotificationThreadGetBuilder1 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn notify_get_repo_list() -> NotificationThreadGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        NotificationThreadGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<NotificationThread> for NotificationThreadBuilder {
    fn into(self) -> NotificationThread {
        self.body
    }
}

/// Builder for [`NotificationThread`](./struct.NotificationThread.html) object.
#[derive(Debug, Clone)]
pub struct NotificationThreadBuilder {
    body: self::NotificationThread,
}

impl NotificationThreadBuilder {
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn pinned(mut self, value: impl Into<bool>) -> Self {
        self.body.pinned = Some(value.into());
        self
    }

    #[inline]
    pub fn repository(mut self, value: crate::repository::Repository) -> Self {
        self.body.repository = Some(value.into());
        self
    }

    #[inline]
    pub fn subject(mut self, value: crate::notification_subject::NotificationSubject) -> Self {
        self.body.subject = Some(value.into());
        self
    }

    #[inline]
    pub fn unread(mut self, value: impl Into<bool>) -> Self {
        self.body.unread = Some(value.into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`NotificationThread::notify_get_list`](./struct.NotificationThread.html#method.notify_get_list) method for a `GET` operation associated with `NotificationThread`.
#[derive(Debug, Clone)]
pub struct NotificationThreadGetBuilder {
    param_all: Option<String>,
    param_status_types: Option<crate::util::Delimited<String, crate::util::Multi>>,
    param_since: Option<String>,
    param_before: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl NotificationThreadGetBuilder {
    /// If true, show notifications marked as read. Default value is false
    #[inline]
    pub fn all(mut self, value: impl Into<String>) -> Self {
        self.param_all = Some(value.into());
        self
    }

    /// Show notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread & pinned.
    #[inline]
    pub fn status_types(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.param_status_types = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// Only show notifications updated after the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.param_since = Some(value.into());
        self
    }

    /// Only show notifications updated before the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.param_before = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for NotificationThreadGetBuilder {
    type Output = Vec<NotificationThread>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/notifications".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("all", self.param_all.as_ref().map(std::string::ToString::to_string)),
            ("since", self.param_since.as_ref().map(std::string::ToString::to_string)),
            ("before", self.param_before.as_ref().map(std::string::ToString::to_string)),
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ])
        .query({
            &self.param_status_types.as_ref().map(|v| {
                v.iter().map(|v| ("status-types", v.to_string())).collect::<Vec<_>>()
            }).unwrap_or_default()
        }))
    }
}

/// Builder created by [`NotificationThread::notify_get_thread`](./struct.NotificationThread.html#method.notify_get_thread) method for a `GET` operation associated with `NotificationThread`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct NotificationThreadGetBuilder1<Id> {
    inner: NotificationThreadGetBuilder1Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct NotificationThreadGetBuilder1Container {
    param_id: Option<String>,
}

impl<Id> NotificationThreadGetBuilder1<Id> {
    /// id of notification thread
    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> NotificationThreadGetBuilder1<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for NotificationThreadGetBuilder1<crate::generics::IdExists> {
    type Output = NotificationThread;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/notifications/threads/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

impl crate::client::ResponseWrapper<NotificationThread, NotificationThreadGetBuilder1<crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`NotificationThread::notify_get_repo_list`](./struct.NotificationThread.html#method.notify_get_repo_list) method for a `GET` operation associated with `NotificationThread`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct NotificationThreadGetBuilder2<Owner, Repo> {
    inner: NotificationThreadGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct NotificationThreadGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_all: Option<String>,
    param_status_types: Option<crate::util::Delimited<String, crate::util::Multi>>,
    param_since: Option<String>,
    param_before: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> NotificationThreadGetBuilder2<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> NotificationThreadGetBuilder2<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> NotificationThreadGetBuilder2<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// If true, show notifications marked as read. Default value is false
    #[inline]
    pub fn all(mut self, value: impl Into<String>) -> Self {
        self.inner.param_all = Some(value.into());
        self
    }

    /// Show notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread & pinned
    #[inline]
    pub fn status_types(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.param_status_types = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// Only show notifications updated after the given time. This is a timestamp in RFC 3339 format
    #[inline]
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.inner.param_since = Some(value.into());
        self
    }

    /// Only show notifications updated before the given time. This is a timestamp in RFC 3339 format
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for NotificationThreadGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<NotificationThread>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/notifications", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("all", self.inner.param_all.as_ref().map(std::string::ToString::to_string)),
            ("since", self.inner.param_since.as_ref().map(std::string::ToString::to_string)),
            ("before", self.inner.param_before.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.inner.param_limit.as_ref().map(std::string::ToString::to_string))
        ])
        .query({
            &self.inner.param_status_types.as_ref().map(|v| {
                v.iter().map(|v| ("status-types", v.to_string())).collect::<Vec<_>>()
            }).unwrap_or_default()
        }))
    }
}
