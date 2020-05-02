
/// TrackedTime worked time for an issue / pr
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TrackedTime {
    pub created: Option<String>,
    pub id: Option<i64>,
    pub issue_id: Option<i64>,
    /// Time in seconds
    pub time: Option<i64>,
    pub user_id: Option<i64>,
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
    pub fn issue_tracked_times() -> TrackedTimeGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        TrackedTimeGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
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
        TrackedTimeGetBuilder3
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

    #[inline]
    pub fn user_id(mut self, value: impl Into<i64>) -> Self {
        self.body.user_id = Some(value.into());
        self
    }
}

/// Builder created by [`TrackedTime::issue_tracked_times`](./struct.TrackedTime.html#method.issue_tracked_times) method for a `GET` operation associated with `TrackedTime`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TrackedTimeGetBuilder<Owner, Repo, Id> {
    inner: TrackedTimeGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct TrackedTimeGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> TrackedTimeGetBuilder<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TrackedTimeGetBuilder<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> TrackedTimeGetBuilder<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TrackedTimeGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = Vec<TrackedTime>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{id}/times", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
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
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TrackedTimeGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<TrackedTime>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/times", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
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

/// Builder created by [`TrackedTime::user_current_tracked_times`](./struct.TrackedTime.html#method.user_current_tracked_times) method for a `GET` operation associated with `TrackedTime`.
#[derive(Debug, Clone)]
pub struct TrackedTimeGetBuilder3;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TrackedTimeGetBuilder3 {
    type Output = Vec<TrackedTime>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/times".into()
    }
}
