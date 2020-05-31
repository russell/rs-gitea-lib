
/// Attachment a generic attachment
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub browser_download_url: Option<String>,
    pub created_at: Option<String>,
    pub download_count: Option<i64>,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub size: Option<i64>,
    pub uuid: Option<String>,
}

impl Attachment {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> AttachmentBuilder {
        AttachmentBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_release_attachments() -> AttachmentGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        AttachmentGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_create_release_attachment() -> AttachmentPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId, crate::generics::MissingAttachment> {
        AttachmentPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
            _param_attachment: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_release_attachment() -> AttachmentGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId, crate::generics::MissingAttachmentId> {
        AttachmentGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
            _param_attachment_id: core::marker::PhantomData,
        }
    }
}

impl Into<Attachment> for AttachmentBuilder {
    fn into(self) -> Attachment {
        self.body
    }
}

/// Builder for [`Attachment`](./struct.Attachment.html) object.
#[derive(Debug, Clone)]
pub struct AttachmentBuilder {
    body: self::Attachment,
}

impl AttachmentBuilder {
    #[inline]
    pub fn browser_download_url(mut self, value: impl Into<String>) -> Self {
        self.body.browser_download_url = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn download_count(mut self, value: impl Into<i64>) -> Self {
        self.body.download_count = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn size(mut self, value: impl Into<i64>) -> Self {
        self.body.size = Some(value.into());
        self
    }

    #[inline]
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.body.uuid = Some(value.into());
        self
    }
}

/// Builder created by [`Attachment::repo_list_release_attachments`](./struct.Attachment.html#method.repo_list_release_attachments) method for a `GET` operation associated with `Attachment`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AttachmentGetBuilder<Owner, Repo, Id> {
    inner: AttachmentGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct AttachmentGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> AttachmentGetBuilder<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> AttachmentGetBuilder<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> AttachmentGetBuilder<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the release
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> AttachmentGetBuilder<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for AttachmentGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = Vec<Attachment>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/releases/{id}/assets", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`Attachment::repo_create_release_attachment`](./struct.Attachment.html#method.repo_create_release_attachment) method for a `POST` operation associated with `Attachment`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AttachmentPostBuilder<Owner, Repo, Id, Attachment> {
    inner: AttachmentPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
    _param_attachment: core::marker::PhantomData<Attachment>,
}

#[derive(Debug, Default, Clone)]
struct AttachmentPostBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
    param_name: Option<String>,
    param_attachment: Option<std::path::PathBuf>,
}

impl<Owner, Repo, Id, Attachment> AttachmentPostBuilder<Owner, Repo, Id, Attachment> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> AttachmentPostBuilder<crate::generics::OwnerExists, Repo, Id, Attachment> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> AttachmentPostBuilder<Owner, crate::generics::RepoExists, Id, Attachment> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the release
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> AttachmentPostBuilder<Owner, Repo, crate::generics::IdExists, Attachment> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the attachment
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.inner.param_name = Some(value.into());
        self
    }

    /// attachment to upload
    #[inline]
    pub fn attachment(mut self, value: impl AsRef<std::path::Path>) -> AttachmentPostBuilder<Owner, Repo, Id, crate::generics::AttachmentExists> {
        self.inner.param_attachment = Some(value.as_ref().into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for AttachmentPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists, crate::generics::AttachmentExists> {
    type Output = Attachment;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/releases/{id}/assets", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .multipart_form_data({
            use crate::client::Form;
            let mut form = <Client::Request as Request>::Form::new();
            if let Some(v) = self.inner.param_attachment.as_ref() {
                form = form.file("attachment", v)?;
            }
            form
        })
        .query(&[
            ("name", self.inner.param_name.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Attachment::repo_get_release_attachment`](./struct.Attachment.html#method.repo_get_release_attachment) method for a `GET` operation associated with `Attachment`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AttachmentGetBuilder1<Owner, Repo, Id, AttachmentId> {
    inner: AttachmentGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
    _param_attachment_id: core::marker::PhantomData<AttachmentId>,
}

#[derive(Debug, Default, Clone)]
struct AttachmentGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
    param_attachment_id: Option<i64>,
}

impl<Owner, Repo, Id, AttachmentId> AttachmentGetBuilder1<Owner, Repo, Id, AttachmentId> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> AttachmentGetBuilder1<crate::generics::OwnerExists, Repo, Id, AttachmentId> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> AttachmentGetBuilder1<Owner, crate::generics::RepoExists, Id, AttachmentId> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the release
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> AttachmentGetBuilder1<Owner, Repo, crate::generics::IdExists, AttachmentId> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the attachment to get
    #[inline]
    pub fn attachment_id(mut self, value: impl Into<i64>) -> AttachmentGetBuilder1<Owner, Repo, Id, crate::generics::AttachmentIdExists> {
        self.inner.param_attachment_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for AttachmentGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists, crate::generics::AttachmentIdExists> {
    type Output = Attachment;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/releases/{id}/assets/{attachment_id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?"), attachment_id=self.inner.param_attachment_id.as_ref().expect("missing parameter attachment_id?")).into()
    }
}
