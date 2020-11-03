
/// AddTimeOption options for adding time to an issue
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddTimeOption {
    pub created: Option<String>,
    /// time in seconds
    pub time: i64,
    /// User who spent the time (optional)
    pub user_name: Option<String>,
}

impl AddTimeOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> AddTimeOptionBuilder<crate::generics::MissingTime> {
        AddTimeOptionBuilder {
            body: Default::default(),
            _time: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_add_time() -> AddTimeOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingTime> {
        AddTimeOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _time: core::marker::PhantomData,
        }
    }
}

impl Into<AddTimeOption> for AddTimeOptionBuilder<crate::generics::TimeExists> {
    fn into(self) -> AddTimeOption {
        self.body
    }
}

impl Into<AddTimeOption> for AddTimeOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::TimeExists> {
    fn into(self) -> AddTimeOption {
        self.inner.body
    }
}

/// Builder for [`AddTimeOption`](./struct.AddTimeOption.html) object.
#[derive(Debug, Clone)]
pub struct AddTimeOptionBuilder<Time> {
    body: self::AddTimeOption,
    _time: core::marker::PhantomData<Time>,
}

impl<Time> AddTimeOptionBuilder<Time> {
    #[inline]
    pub fn created(mut self, value: impl Into<String>) -> Self {
        self.body.created = Some(value.into());
        self
    }

    /// time in seconds
    #[inline]
    pub fn time(mut self, value: impl Into<i64>) -> AddTimeOptionBuilder<crate::generics::TimeExists> {
        self.body.time = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// User who spent the time (optional)
    #[inline]
    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.body.user_name = Some(value.into());
        self
    }
}

/// Builder created by [`AddTimeOption::issue_add_time`](./struct.AddTimeOption.html#method.issue_add_time) method for a `POST` operation associated with `AddTimeOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AddTimeOptionPostBuilder<Owner, Repo, Index, Time> {
    inner: AddTimeOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _time: core::marker::PhantomData<Time>,
}

#[derive(Debug, Default, Clone)]
struct AddTimeOptionPostBuilderContainer {
    body: self::AddTimeOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index, Time> AddTimeOptionPostBuilder<Owner, Repo, Index, Time> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> AddTimeOptionPostBuilder<crate::generics::OwnerExists, Repo, Index, Time> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> AddTimeOptionPostBuilder<Owner, crate::generics::RepoExists, Index, Time> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> AddTimeOptionPostBuilder<Owner, Repo, crate::generics::IndexExists, Time> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn created(mut self, value: impl Into<String>) -> Self {
        self.inner.body.created = Some(value.into());
        self
    }

    /// time in seconds
    #[inline]
    pub fn time(mut self, value: impl Into<i64>) -> AddTimeOptionPostBuilder<Owner, Repo, Index, crate::generics::TimeExists> {
        self.inner.body.time = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// User who spent the time (optional)
    #[inline]
    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.user_name = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for AddTimeOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::TimeExists> {
    type Output = crate::tracked_time::TrackedTime;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/times", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::tracked_time::TrackedTime, AddTimeOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::TimeExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
