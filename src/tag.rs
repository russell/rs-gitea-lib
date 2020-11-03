
/// Tag represents a repository tag
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub commit: Option<crate::commit_meta::CommitMeta>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
}

impl Tag {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TagBuilder {
        TagBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_tags() -> TagGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        TagGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<Tag> for TagBuilder {
    fn into(self) -> Tag {
        self.body
    }
}

/// Builder for [`Tag`](./struct.Tag.html) object.
#[derive(Debug, Clone)]
pub struct TagBuilder {
    body: self::Tag,
}

impl TagBuilder {
    #[inline]
    pub fn commit(mut self, value: crate::commit_meta::CommitMeta) -> Self {
        self.body.commit = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn tarball_url(mut self, value: impl Into<String>) -> Self {
        self.body.tarball_url = Some(value.into());
        self
    }

    #[inline]
    pub fn zipball_url(mut self, value: impl Into<String>) -> Self {
        self.body.zipball_url = Some(value.into());
        self
    }
}

/// Builder created by [`Tag::repo_list_tags`](./struct.Tag.html#method.repo_list_tags) method for a `GET` operation associated with `Tag`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TagGetBuilder<Owner, Repo> {
    inner: TagGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct TagGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> TagGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TagGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TagGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }

    /// page size of results, default maximum page size is 50
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TagGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Tag>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/tags", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
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
