
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
    pub fn user_current_check_subscription() -> WatchInfoGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        WatchInfoGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_current_put_subscription() -> WatchInfoPutBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        WatchInfoPutBuilder {
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

/// Builder created by [`WatchInfo::user_current_check_subscription`](./struct.WatchInfo.html#method.user_current_check_subscription) method for a `GET` operation associated with `WatchInfo`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct WatchInfoGetBuilder<Owner, Repo> {
    inner: WatchInfoGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct WatchInfoGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> WatchInfoGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> WatchInfoGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> WatchInfoGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for WatchInfoGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = WatchInfo;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/subscription", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`WatchInfo::user_current_put_subscription`](./struct.WatchInfo.html#method.user_current_put_subscription) method for a `PUT` operation associated with `WatchInfo`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct WatchInfoPutBuilder<Owner, Repo> {
    inner: WatchInfoPutBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct WatchInfoPutBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> WatchInfoPutBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> WatchInfoPutBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> WatchInfoPutBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for WatchInfoPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = WatchInfo;

    const METHOD: http::Method = http::Method::PUT;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/subscription", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

