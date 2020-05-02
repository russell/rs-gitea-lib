#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub object: Option<crate::git_object::GitObject>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub url: Option<String>,
}

impl Reference {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ReferenceBuilder {
        ReferenceBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_all_git_refs() -> ReferenceGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        ReferenceGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_list_git_refs() -> ReferenceGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingRef> {
        ReferenceGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_ref: core::marker::PhantomData,
        }
    }
}

impl Into<Reference> for ReferenceBuilder {
    fn into(self) -> Reference {
        self.body
    }
}

/// Builder for [`Reference`](./struct.Reference.html) object.
#[derive(Debug, Clone)]
pub struct ReferenceBuilder {
    body: self::Reference,
}

impl ReferenceBuilder {
    #[inline]
    pub fn object(mut self, value: crate::git_object::GitObject) -> Self {
        self.body.object = Some(value.into());
        self
    }

    #[inline]
    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.body.ref_ = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`Reference::repo_list_all_git_refs`](./struct.Reference.html#method.repo_list_all_git_refs) method for a `GET` operation associated with `Reference`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ReferenceGetBuilder<Owner, Repo> {
    inner: ReferenceGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct ReferenceGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> ReferenceGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> ReferenceGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> ReferenceGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ReferenceGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Reference>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/git/refs", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`Reference::repo_list_git_refs`](./struct.Reference.html#method.repo_list_git_refs) method for a `GET` operation associated with `Reference`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ReferenceGetBuilder1<Owner, Repo, Ref> {
    inner: ReferenceGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_ref: core::marker::PhantomData<Ref>,
}

#[derive(Debug, Default, Clone)]
struct ReferenceGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_ref: Option<String>,
}

impl<Owner, Repo, Ref> ReferenceGetBuilder1<Owner, Repo, Ref> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> ReferenceGetBuilder1<crate::generics::OwnerExists, Repo, Ref> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> ReferenceGetBuilder1<Owner, crate::generics::RepoExists, Ref> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// part or full name of the ref
    #[inline]
    pub fn r#ref(mut self, value: impl Into<String>) -> ReferenceGetBuilder1<Owner, Repo, crate::generics::RefExists> {
        self.inner.param_ref = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ReferenceGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::RefExists> {
    type Output = Vec<Reference>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/git/refs/{ref}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), ref=self.inner.param_ref.as_ref().expect("missing parameter ref?")).into()
    }
}
