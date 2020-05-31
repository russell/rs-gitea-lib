
/// EditReactionOption contain the reaction type
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditReactionOption {
    pub content: Option<String>,
}

impl EditReactionOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditReactionOptionBuilder {
        EditReactionOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_post_comment_reaction() -> EditReactionOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        EditReactionOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_delete_comment_reaction() -> EditReactionOptionDeleteBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        EditReactionOptionDeleteBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_post_issue_reaction() -> EditReactionOptionPostBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        EditReactionOptionPostBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_delete_issue_reaction() -> EditReactionOptionDeleteBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        EditReactionOptionDeleteBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<EditReactionOption> for EditReactionOptionBuilder {
    fn into(self) -> EditReactionOption {
        self.body
    }
}

impl Into<EditReactionOption> for EditReactionOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    fn into(self) -> EditReactionOption {
        self.inner.body
    }
}

impl Into<EditReactionOption> for EditReactionOptionDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    fn into(self) -> EditReactionOption {
        self.inner.body
    }
}

impl Into<EditReactionOption> for EditReactionOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> EditReactionOption {
        self.inner.body
    }
}

impl Into<EditReactionOption> for EditReactionOptionDeleteBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    fn into(self) -> EditReactionOption {
        self.inner.body
    }
}

/// Builder for [`EditReactionOption`](./struct.EditReactionOption.html) object.
#[derive(Debug, Clone)]
pub struct EditReactionOptionBuilder {
    body: self::EditReactionOption,
}

impl EditReactionOptionBuilder {
    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.body.content = Some(value.into());
        self
    }
}

/// Builder created by [`EditReactionOption::issue_post_comment_reaction`](./struct.EditReactionOption.html#method.issue_post_comment_reaction) method for a `POST` operation associated with `EditReactionOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditReactionOptionPostBuilder<Owner, Repo, Id> {
    inner: EditReactionOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditReactionOptionPostBuilderContainer {
    body: self::EditReactionOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> EditReactionOptionPostBuilder<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditReactionOptionPostBuilder<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditReactionOptionPostBuilder<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the comment to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditReactionOptionPostBuilder<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.inner.body.content = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditReactionOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = crate::reaction_response::ReactionResponse;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/comments/{id}/reactions", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::reaction_response::ReactionResponse, EditReactionOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`EditReactionOption::issue_delete_comment_reaction`](./struct.EditReactionOption.html#method.issue_delete_comment_reaction) method for a `DELETE` operation associated with `EditReactionOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditReactionOptionDeleteBuilder<Owner, Repo, Id> {
    inner: EditReactionOptionDeleteBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditReactionOptionDeleteBuilderContainer {
    body: self::EditReactionOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> EditReactionOptionDeleteBuilder<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditReactionOptionDeleteBuilder<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditReactionOptionDeleteBuilder<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the comment to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> EditReactionOptionDeleteBuilder<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.inner.body.content = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditReactionOptionDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/comments/{id}/reactions", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}

impl crate::client::ResponseWrapper<serde_json::Value, EditReactionOptionDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`EditReactionOption::issue_post_issue_reaction`](./struct.EditReactionOption.html#method.issue_post_issue_reaction) method for a `POST` operation associated with `EditReactionOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditReactionOptionPostBuilder1<Owner, Repo, Index> {
    inner: EditReactionOptionPostBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct EditReactionOptionPostBuilder1Container {
    body: self::EditReactionOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> EditReactionOptionPostBuilder1<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditReactionOptionPostBuilder1<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditReactionOptionPostBuilder1<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> EditReactionOptionPostBuilder1<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.inner.body.content = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditReactionOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = crate::reaction_response::ReactionResponse;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/reactions", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::reaction_response::ReactionResponse, EditReactionOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`EditReactionOption::issue_delete_issue_reaction`](./struct.EditReactionOption.html#method.issue_delete_issue_reaction) method for a `DELETE` operation associated with `EditReactionOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditReactionOptionDeleteBuilder1<Owner, Repo, Index> {
    inner: EditReactionOptionDeleteBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct EditReactionOptionDeleteBuilder1Container {
    body: self::EditReactionOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> EditReactionOptionDeleteBuilder1<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditReactionOptionDeleteBuilder1<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditReactionOptionDeleteBuilder1<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> EditReactionOptionDeleteBuilder1<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.inner.body.content = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditReactionOptionDeleteBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/reactions", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}

impl crate::client::ResponseWrapper<serde_json::Value, EditReactionOptionDeleteBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
