
/// AddTimeOption options for adding time to an issue
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddTimeOption {
    /// time in seconds
    pub time: i64,
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
    pub fn issue_add_time() -> AddTimeOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId, crate::generics::MissingTime> {
        AddTimeOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
            _time: core::marker::PhantomData,
        }
    }
}

impl Into<AddTimeOption> for AddTimeOptionBuilder<crate::generics::TimeExists> {
    fn into(self) -> AddTimeOption {
        self.body
    }
}

impl Into<AddTimeOption> for AddTimeOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists, crate::generics::TimeExists> {
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
    /// time in seconds
    #[inline]
    pub fn time(mut self, value: impl Into<i64>) -> AddTimeOptionBuilder<crate::generics::TimeExists> {
        self.body.time = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`AddTimeOption::issue_add_time`](./struct.AddTimeOption.html#method.issue_add_time) method for a `POST` operation associated with `AddTimeOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AddTimeOptionPostBuilder<Owner, Repo, Id, Time> {
    inner: AddTimeOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
    _time: core::marker::PhantomData<Time>,
}

#[derive(Debug, Default, Clone)]
struct AddTimeOptionPostBuilderContainer {
    body: self::AddTimeOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id, Time> AddTimeOptionPostBuilder<Owner, Repo, Id, Time> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> AddTimeOptionPostBuilder<crate::generics::OwnerExists, Repo, Id, Time> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> AddTimeOptionPostBuilder<Owner, crate::generics::RepoExists, Id, Time> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue to add tracked time to
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> AddTimeOptionPostBuilder<Owner, Repo, crate::generics::IdExists, Time> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// time in seconds
    #[inline]
    pub fn time(mut self, value: impl Into<i64>) -> AddTimeOptionPostBuilder<Owner, Repo, Id, crate::generics::TimeExists> {
        self.inner.body.time = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for AddTimeOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists, crate::generics::TimeExists> {
    type Output = crate::tracked_time::TrackedTime;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{id}/times", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
