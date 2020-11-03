
/// Reaction contain one reaction
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub content: Option<String>,
    pub created_at: Option<String>,
    pub user: Option<crate::user::User>,
}

impl Reaction {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ReactionBuilder {
        ReactionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn issue_get_comment_reactions() -> ReactionGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        ReactionGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_get_issue_reactions() -> ReactionGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        ReactionGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }
}

impl Into<Reaction> for ReactionBuilder {
    fn into(self) -> Reaction {
        self.body
    }
}

/// Builder for [`Reaction`](./struct.Reaction.html) object.
#[derive(Debug, Clone)]
pub struct ReactionBuilder {
    body: self::Reaction,
}

impl ReactionBuilder {
    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.body.content = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn user(mut self, value: crate::user::User) -> Self {
        self.body.user = Some(value.into());
        self
    }
}

/// Builder created by [`Reaction::issue_get_comment_reactions`](./struct.Reaction.html#method.issue_get_comment_reactions) method for a `GET` operation associated with `Reaction`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ReactionGetBuilder<Owner, Repo, Id> {
    inner: ReactionGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct ReactionGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<i64>,
}

impl<Owner, Repo, Id> ReactionGetBuilder<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> ReactionGetBuilder<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> ReactionGetBuilder<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the comment to edit
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> ReactionGetBuilder<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ReactionGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = Vec<Reaction>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/comments/{id}/reactions", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

impl crate::client::ResponseWrapper<Vec<Reaction>, ReactionGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`Reaction::issue_get_issue_reactions`](./struct.Reaction.html#method.issue_get_issue_reactions) method for a `GET` operation associated with `Reaction`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ReactionGetBuilder1<Owner, Repo, Index> {
    inner: ReactionGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct ReactionGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo, Index> ReactionGetBuilder1<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> ReactionGetBuilder1<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> ReactionGetBuilder1<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> ReactionGetBuilder1<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ReactionGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<Reaction>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/reactions", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
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

impl crate::client::ResponseWrapper<Vec<Reaction>, ReactionGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
