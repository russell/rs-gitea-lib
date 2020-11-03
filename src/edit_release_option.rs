
/// EditReleaseOption options when editing a release
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditReleaseOption {
    pub body: Option<String>,
    pub draft: Option<bool>,
    pub name: Option<String>,
    pub prerelease: Option<bool>,
    pub tag_name: Option<String>,
    pub target_commitish: Option<String>,
}

impl EditReleaseOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditReleaseOptionBuilder {
        EditReleaseOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_edit_release() -> EditReleaseOptionPatchBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        EditReleaseOptionPatchBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<EditReleaseOption> for EditReleaseOptionBuilder {
    fn into(self) -> EditReleaseOption {
        self.body
    }
}

impl Into<EditReleaseOption> for EditReleaseOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    fn into(self) -> EditReleaseOption {
        self.inner.body
    }
}

/// Builder for [`EditReleaseOption`](./struct.EditReleaseOption.html) object.
#[derive(Debug, Clone)]
pub struct EditReleaseOptionBuilder {
    body: self::EditReleaseOption,
}

impl EditReleaseOptionBuilder {
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
    pub fn tag_name(mut self, value: impl Into<String>) -> Self {
        self.body.tag_name = Some(value.into());
        self
    }

    #[inline]
    pub fn target_commitish(mut self, value: impl Into<String>) -> Self {
        self.body.target_commitish = Some(value.into());
        self
    }
}

/// Builder created by [`EditReleaseOption::repo_edit_release`](./struct.EditReleaseOption.html#method.repo_edit_release) method for a `PATCH` operation associated with `EditReleaseOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditReleaseOptionPatchBuilder<Owner, Repo, Id> {
    inner: EditReleaseOptionPatchBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditReleaseOptionPatchBuilderContainer {
    body: self::EditReleaseOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> EditReleaseOptionPatchBuilder<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditReleaseOptionPatchBuilder<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditReleaseOptionPatchBuilder<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the release to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditReleaseOptionPatchBuilder<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
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
    pub fn tag_name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.tag_name = Some(value.into());
        self
    }

    #[inline]
    pub fn target_commitish(mut self, value: impl Into<String>) -> Self {
        self.inner.body.target_commitish = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditReleaseOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = crate::release::Release;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/releases/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
