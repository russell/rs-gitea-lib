
/// TopicName a list of repo topic names
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TopicName {
    pub topics: Option<Vec<String>>,
}

impl TopicName {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TopicNameBuilder {
        TopicNameBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_topics() -> TopicNameGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        TopicNameGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<TopicName> for TopicNameBuilder {
    fn into(self) -> TopicName {
        self.body
    }
}

/// Builder for [`TopicName`](./struct.TopicName.html) object.
#[derive(Debug, Clone)]
pub struct TopicNameBuilder {
    body: self::TopicName,
}

impl TopicNameBuilder {
    #[inline]
    pub fn topics(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.topics = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`TopicName::repo_list_topics`](./struct.TopicName.html#method.repo_list_topics) method for a `GET` operation associated with `TopicName`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TopicNameGetBuilder<Owner, Repo> {
    inner: TopicNameGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct TopicNameGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> TopicNameGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TopicNameGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TopicNameGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TopicNameGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = TopicName;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/topics", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
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
