
/// EditAttachmentOptions options for editing attachments
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditAttachmentOptions {
    pub name: Option<String>,
}

impl EditAttachmentOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditAttachmentOptionsBuilder {
        EditAttachmentOptionsBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_edit_release_attachment() -> EditAttachmentOptionsPatchBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId, crate::generics::MissingAttachmentId> {
        EditAttachmentOptionsPatchBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
            _param_attachment_id: core::marker::PhantomData,
        }
    }
}

impl Into<EditAttachmentOptions> for EditAttachmentOptionsBuilder {
    fn into(self) -> EditAttachmentOptions {
        self.body
    }
}

impl Into<EditAttachmentOptions> for EditAttachmentOptionsPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists, crate::generics::AttachmentIdExists> {
    fn into(self) -> EditAttachmentOptions {
        self.inner.body
    }
}

/// Builder for [`EditAttachmentOptions`](./struct.EditAttachmentOptions.html) object.
#[derive(Debug, Clone)]
pub struct EditAttachmentOptionsBuilder {
    body: self::EditAttachmentOptions,
}

impl EditAttachmentOptionsBuilder {
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }
}

/// Builder created by [`EditAttachmentOptions::repo_edit_release_attachment`](./struct.EditAttachmentOptions.html#method.repo_edit_release_attachment) method for a `PATCH` operation associated with `EditAttachmentOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditAttachmentOptionsPatchBuilder<Owner, Repo, Id, AttachmentId> {
    inner: EditAttachmentOptionsPatchBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
    _param_attachment_id: core::marker::PhantomData<AttachmentId>,
}

#[derive(Debug, Default, Clone)]
struct EditAttachmentOptionsPatchBuilderContainer {
    body: self::EditAttachmentOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
    param_attachment_id: Option<i64>,
}

impl<Owner, Repo, Id, AttachmentId> EditAttachmentOptionsPatchBuilder<Owner, Repo, Id, AttachmentId> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditAttachmentOptionsPatchBuilder<crate::generics::OwnerExists, Repo, Id, AttachmentId> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditAttachmentOptionsPatchBuilder<Owner, crate::generics::RepoExists, Id, AttachmentId> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the release
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditAttachmentOptionsPatchBuilder<Owner, Repo, crate::generics::IdExists, AttachmentId> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the attachment to edit
    #[inline]
    pub fn attachment_id(mut self, value: impl Into<i64>) -> EditAttachmentOptionsPatchBuilder<Owner, Repo, Id, crate::generics::AttachmentIdExists> {
        self.inner.param_attachment_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.name = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditAttachmentOptionsPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists, crate::generics::AttachmentIdExists> {
    type Output = crate::patch_repos_owner_repo_releases_id_assets_attachment_id_response::PatchReposOwnerRepoReleasesIdAssetsAttachmentIdResponse;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/releases/{id}/assets/{attachment_id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?"), attachment_id=self.inner.param_attachment_id.as_ref().expect("missing parameter attachment_id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
