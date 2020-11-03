
/// WatchInfo represents an API watch status of one repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WatchInfo {
    pub created_at: Option<String>,
    pub ignored: Option<bool>,
    pub reason: Option<crate::watch_info::WatchInfoReason>,
    pub repository_url: Option<String>,
    pub subscribed: Option<bool>,
    pub url: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WatchInfoReason {}

impl WatchInfo {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> WatchInfoBuilder {
        WatchInfoBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_check_subscription() -> WatchInfoGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        WatchInfoGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_current_check_subscription() -> WatchInfoGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        WatchInfoGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_current_put_subscription() -> WatchInfoPutBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        WatchInfoPutBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<WatchInfo> for WatchInfoBuilder {
    fn into(self) -> WatchInfo {
        self.body
    }
}

/// Builder for [`WatchInfo`](./struct.WatchInfo.html) object.
#[derive(Debug, Clone)]
pub struct WatchInfoBuilder {
    body: self::WatchInfo,
}

impl WatchInfoBuilder {
    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn ignored(mut self, value: impl Into<bool>) -> Self {
        self.body.ignored = Some(value.into());
        self
    }

    #[inline]
    pub fn reason(mut self, value: crate::watch_info::WatchInfoReason) -> Self {
        self.body.reason = Some(value.into());
        self
    }

    #[inline]
    pub fn repository_url(mut self, value: impl Into<String>) -> Self {
        self.body.repository_url = Some(value.into());
        self
    }

    #[inline]
    pub fn subscribed(mut self, value: impl Into<bool>) -> Self {
        self.body.subscribed = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`WatchInfo::issue_check_subscription`](./struct.WatchInfo.html#method.issue_check_subscription) method for a `GET` operation associated with `WatchInfo`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct WatchInfoGetBuilder<Owner, Repo, Index> {
    inner: WatchInfoGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct WatchInfoGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> WatchInfoGetBuilder<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> WatchInfoGetBuilder<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> WatchInfoGetBuilder<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> WatchInfoGetBuilder<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for WatchInfoGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = WatchInfo;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/subscriptions/check", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }
}

/// Builder created by [`WatchInfo::user_current_check_subscription`](./struct.WatchInfo.html#method.user_current_check_subscription) method for a `GET` operation associated with `WatchInfo`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct WatchInfoGetBuilder1<Owner, Repo> {
    inner: WatchInfoGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct WatchInfoGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> WatchInfoGetBuilder1<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> WatchInfoGetBuilder1<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> WatchInfoGetBuilder1<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for WatchInfoGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = WatchInfo;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/subscription", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`WatchInfo::user_current_put_subscription`](./struct.WatchInfo.html#method.user_current_put_subscription) method for a `PUT` operation associated with `WatchInfo`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct WatchInfoPutBuilder1<Owner, Repo> {
    inner: WatchInfoPutBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct WatchInfoPutBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> WatchInfoPutBuilder1<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> WatchInfoPutBuilder1<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> WatchInfoPutBuilder1<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for WatchInfoPutBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = WatchInfo;

    const METHOD: http::Method = http::Method::PUT;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/subscription", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

