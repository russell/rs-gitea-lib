
/// CreateReleaseOption options when creating a release
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateReleaseOption {
    pub body: Option<String>,
    pub draft: Option<bool>,
    pub name: Option<String>,
    pub prerelease: Option<bool>,
    pub tag_name: String,
    pub target_commitish: Option<String>,
}

impl CreateReleaseOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateReleaseOptionBuilder<crate::generics::MissingTagName> {
        CreateReleaseOptionBuilder {
            body: Default::default(),
            _tag_name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_create_release() -> CreateReleaseOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingTagName> {
        CreateReleaseOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _tag_name: core::marker::PhantomData,
        }
    }
}

impl Into<CreateReleaseOption> for CreateReleaseOptionBuilder<crate::generics::TagNameExists> {
    fn into(self) -> CreateReleaseOption {
        self.body
    }
}

impl Into<CreateReleaseOption> for CreateReleaseOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::TagNameExists> {
    fn into(self) -> CreateReleaseOption {
        self.inner.body
    }
}

/// Builder for [`CreateReleaseOption`](./struct.CreateReleaseOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateReleaseOptionBuilder<TagName> {
    body: self::CreateReleaseOption,
    _tag_name: core::marker::PhantomData<TagName>,
}

impl<TagName> CreateReleaseOptionBuilder<TagName> {
    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn draft(mut self, value: impl Into<bool>) -> Self {
        self.body.draft = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn prerelease(mut self, value: impl Into<bool>) -> Self {
        self.body.prerelease = Some(value.into());
        self
    }

    #[inline]
    pub fn tag_name(mut self, value: impl Into<String>) -> CreateReleaseOptionBuilder<crate::generics::TagNameExists> {
        self.body.tag_name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn target_commitish(mut self, value: impl Into<String>) -> Self {
        self.body.target_commitish = Some(value.into());
        self
    }
}

/// Builder created by [`CreateReleaseOption::repo_create_release`](./struct.CreateReleaseOption.html#method.repo_create_release) method for a `POST` operation associated with `CreateReleaseOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateReleaseOptionPostBuilder<Owner, Repo, TagName> {
    inner: CreateReleaseOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _tag_name: core::marker::PhantomData<TagName>,
}

#[derive(Debug, Default, Clone)]
struct CreateReleaseOptionPostBuilderContainer {
    body: self::CreateReleaseOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo, TagName> CreateReleaseOptionPostBuilder<Owner, Repo, TagName> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateReleaseOptionPostBuilder<crate::generics::OwnerExists, Repo, TagName> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateReleaseOptionPostBuilder<Owner, crate::generics::RepoExists, TagName> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.inner.body.body = Some(value.into());
        self
    }

    #[inline]
    pub fn draft(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.draft = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn prerelease(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.prerelease = Some(value.into());
        self
    }

    #[inline]
    pub fn tag_name(mut self, value: impl Into<String>) -> CreateReleaseOptionPostBuilder<Owner, Repo, crate::generics::TagNameExists> {
        self.inner.body.tag_name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn target_commitish(mut self, value: impl Into<String>) -> Self {
        self.inner.body.target_commitish = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateReleaseOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::TagNameExists> {
    type Output = crate::release::Release;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/releases", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
