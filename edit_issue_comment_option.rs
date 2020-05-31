
/// EditIssueCommentOption options for editing a comment
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditIssueCommentOption {
    pub body: String,
}

impl EditIssueCommentOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditIssueCommentOptionBuilder<crate::generics::MissingBody> {
        EditIssueCommentOptionBuilder {
            body: Default::default(),
            _body: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_edit_comment() -> EditIssueCommentOptionPatchBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId, crate::generics::MissingBody> {
        EditIssueCommentOptionPatchBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
            _body: core::marker::PhantomData,
        }
    }

    #[deprecated]
    #[inline]
    pub fn issue_edit_comment_deprecated() -> EditIssueCommentOptionPatchBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingId, crate::generics::MissingBody> {
        EditIssueCommentOptionPatchBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
            _body: core::marker::PhantomData,
        }
    }
}

impl Into<EditIssueCommentOption> for EditIssueCommentOptionBuilder<crate::generics::BodyExists> {
    fn into(self) -> EditIssueCommentOption {
        self.body
    }
}

impl Into<EditIssueCommentOption> for EditIssueCommentOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists, crate::generics::BodyExists> {
    fn into(self) -> EditIssueCommentOption {
        self.inner.body
    }
}

impl Into<EditIssueCommentOption> for EditIssueCommentOptionPatchBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists, crate::generics::BodyExists> {
    fn into(self) -> EditIssueCommentOption {
        self.inner.body
    }
}

/// Builder for [`EditIssueCommentOption`](./struct.EditIssueCommentOption.html) object.
#[derive(Debug, Clone)]
pub struct EditIssueCommentOptionBuilder<Body> {
    body: self::EditIssueCommentOption,
    _body: core::marker::PhantomData<Body>,
}

impl<Body> EditIssueCommentOptionBuilder<Body> {
    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> EditIssueCommentOptionBuilder<crate::generics::BodyExists> {
        self.body.body = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`EditIssueCommentOption::issue_edit_comment`](./struct.EditIssueCommentOption.html#method.issue_edit_comment) method for a `PATCH` operation associated with `EditIssueCommentOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditIssueCommentOptionPatchBuilder<Owner, Repo, Id, Body> {
    inner: EditIssueCommentOptionPatchBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
    _body: core::marker::PhantomData<Body>,
}

#[derive(Debug, Default, Clone)]
struct EditIssueCommentOptionPatchBuilderContainer {
    body: self::EditIssueCommentOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id, Body> EditIssueCommentOptionPatchBuilder<Owner, Repo, Id, Body> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditIssueCommentOptionPatchBuilder<crate::generics::OwnerExists, Repo, Id, Body> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditIssueCommentOptionPatchBuilder<Owner, crate::generics::RepoExists, Id, Body> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the comment to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditIssueCommentOptionPatchBuilder<Owner, Repo, crate::generics::IdExists, Body> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> EditIssueCommentOptionPatchBuilder<Owner, Repo, Id, crate::generics::BodyExists> {
        self.inner.body.body = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditIssueCommentOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists, crate::generics::BodyExists> {
    type Output = crate::comment::Comment;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/comments/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

/// Builder created by [`EditIssueCommentOption::issue_edit_comment_deprecated`](./struct.EditIssueCommentOption.html#method.issue_edit_comment_deprecated) method for a `PATCH` operation associated with `EditIssueCommentOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditIssueCommentOptionPatchBuilder1<Owner, Repo, Index, Id, Body> {
    inner: EditIssueCommentOptionPatchBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _param_id: core::marker::PhantomData<Id>,
    _body: core::marker::PhantomData<Body>,
}

#[derive(Debug, Default, Clone)]
struct EditIssueCommentOptionPatchBuilder1Container {
    body: self::EditIssueCommentOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Index, Id, Body> EditIssueCommentOptionPatchBuilder1<Owner, Repo, Index, Id, Body> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditIssueCommentOptionPatchBuilder1<crate::generics::OwnerExists, Repo, Index, Id, Body> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditIssueCommentOptionPatchBuilder1<Owner, crate::generics::RepoExists, Index, Id, Body> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// this parameter is ignored
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> EditIssueCommentOptionPatchBuilder1<Owner, Repo, crate::generics::IndexExists, Id, Body> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the comment to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditIssueCommentOptionPatchBuilder1<Owner, Repo, Index, crate::generics::IdExists, Body> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> EditIssueCommentOptionPatchBuilder1<Owner, Repo, Index, Id, crate::generics::BodyExists> {
        self.inner.body.body = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditIssueCommentOptionPatchBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::IdExists, crate::generics::BodyExists> {
    type Output = crate::comment::Comment;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/comments/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
