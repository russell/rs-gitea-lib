
/// RepoTopicOptions a collection of repo topic names
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RepoTopicOptions {
    /// list of topic names
    pub topics: Option<Vec<String>>,
}

impl RepoTopicOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> RepoTopicOptionsBuilder {
        RepoTopicOptionsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_update_topics() -> RepoTopicOptionsPutBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        RepoTopicOptionsPutBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<RepoTopicOptions> for RepoTopicOptionsBuilder {
    fn into(self) -> RepoTopicOptions {
        self.body
    }
}

impl Into<RepoTopicOptions> for RepoTopicOptionsPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    fn into(self) -> RepoTopicOptions {
        self.inner.body
    }
}

/// Builder for [`RepoTopicOptions`](./struct.RepoTopicOptions.html) object.
#[derive(Debug, Clone)]
pub struct RepoTopicOptionsBuilder {
    body: self::RepoTopicOptions,
}

impl RepoTopicOptionsBuilder {
    /// list of topic names
    #[inline]
    pub fn topics(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.topics = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`RepoTopicOptions::repo_update_topics`](./struct.RepoTopicOptions.html#method.repo_update_topics) method for a `PUT` operation associated with `RepoTopicOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepoTopicOptionsPutBuilder<Owner, Repo> {
    inner: RepoTopicOptionsPutBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct RepoTopicOptionsPutBuilderContainer {
    body: self::RepoTopicOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> RepoTopicOptionsPutBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> RepoTopicOptionsPutBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> RepoTopicOptionsPutBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// list of topic names
    #[inline]
    pub fn topics(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.topics = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepoTopicOptionsPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::PUT;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/topics", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}
