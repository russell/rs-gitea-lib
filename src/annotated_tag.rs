
/// AnnotatedTag represents an annotated tag
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnnotatedTag {
    pub message: Option<String>,
    pub object: Option<crate::annotated_tag_object::AnnotatedTagObject>,
    pub sha: Option<String>,
    pub tag: Option<String>,
    pub tagger: Option<crate::commit_user::CommitUser>,
    pub url: Option<String>,
    pub verification: Option<crate::payload_commit_verification::PayloadCommitVerification>,
}

impl AnnotatedTag {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> AnnotatedTagBuilder {
        AnnotatedTagBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get_tag() -> AnnotatedTagGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingSha> {
        AnnotatedTagGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_sha: core::marker::PhantomData,
        }
    }
}

impl Into<AnnotatedTag> for AnnotatedTagBuilder {
    fn into(self) -> AnnotatedTag {
        self.body
    }
}

/// Builder for [`AnnotatedTag`](./struct.AnnotatedTag.html) object.
#[derive(Debug, Clone)]
pub struct AnnotatedTagBuilder {
    body: self::AnnotatedTag,
}

impl AnnotatedTagBuilder {
    #[inline]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.body.message = Some(value.into());
        self
    }

    #[inline]
    pub fn object(mut self, value: crate::annotated_tag_object::AnnotatedTagObject) -> Self {
        self.body.object = Some(value.into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }

    #[inline]
    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.body.tag = Some(value.into());
        self
    }

    #[inline]
    pub fn tagger(mut self, value: crate::commit_user::CommitUser) -> Self {
        self.body.tagger = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }

    #[inline]
    pub fn verification(mut self, value: crate::payload_commit_verification::PayloadCommitVerification) -> Self {
        self.body.verification = Some(value.into());
        self
    }
}

/// Builder created by [`AnnotatedTag::get_tag`](./struct.AnnotatedTag.html#method.get_tag) method for a `GET` operation associated with `AnnotatedTag`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AnnotatedTagGetBuilder<Owner, Repo, Sha> {
    inner: AnnotatedTagGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_sha: core::marker::PhantomData<Sha>,
}

#[derive(Debug, Default, Clone)]
struct AnnotatedTagGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_sha: Option<String>,
}

impl<Owner, Repo, Sha> AnnotatedTagGetBuilder<Owner, Repo, Sha> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> AnnotatedTagGetBuilder<crate::generics::OwnerExists, Repo, Sha> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> AnnotatedTagGetBuilder<Owner, crate::generics::RepoExists, Sha> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// sha of the tag. The Git tags API only supports annotated tag objects, not lightweight tags.
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> AnnotatedTagGetBuilder<Owner, Repo, crate::generics::ShaExists> {
        self.inner.param_sha = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for AnnotatedTagGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists> {
    type Output = AnnotatedTag;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/git/tags/{sha}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), sha=self.inner.param_sha.as_ref().expect("missing parameter sha?")).into()
    }
}

impl crate::client::ResponseWrapper<AnnotatedTag, AnnotatedTagGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
