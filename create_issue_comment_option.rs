
/// CreateIssueCommentOption options for creating a comment on an issue
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateIssueCommentOption {
    pub body: String,
}

impl CreateIssueCommentOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateIssueCommentOptionBuilder<crate::generics::MissingBody> {
        CreateIssueCommentOptionBuilder {
            body: Default::default(),
            _body: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_create_comment() -> CreateIssueCommentOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingBody> {
        CreateIssueCommentOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _body: core::marker::PhantomData,
        }
    }
}

impl Into<CreateIssueCommentOption> for CreateIssueCommentOptionBuilder<crate::generics::BodyExists> {
    fn into(self) -> CreateIssueCommentOption {
        self.body
    }
}

impl Into<CreateIssueCommentOption> for CreateIssueCommentOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::BodyExists> {
    fn into(self) -> CreateIssueCommentOption {
        self.inner.body
    }
}

/// Builder for [`CreateIssueCommentOption`](./struct.CreateIssueCommentOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateIssueCommentOptionBuilder<Body> {
    body: self::CreateIssueCommentOption,
    _body: core::marker::PhantomData<Body>,
}

impl<Body> CreateIssueCommentOptionBuilder<Body> {
    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> CreateIssueCommentOptionBuilder<crate::generics::BodyExists> {
        self.body.body = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`CreateIssueCommentOption::issue_create_comment`](./struct.CreateIssueCommentOption.html#method.issue_create_comment) method for a `POST` operation associated with `CreateIssueCommentOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateIssueCommentOptionPostBuilder<Owner, Repo, Index, Body> {
    inner: CreateIssueCommentOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _body: core::marker::PhantomData<Body>,
}

#[derive(Debug, Default, Clone)]
struct CreateIssueCommentOptionPostBuilderContainer {
    body: self::CreateIssueCommentOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index, Body> CreateIssueCommentOptionPostBuilder<Owner, Repo, Index, Body> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateIssueCommentOptionPostBuilder<crate::generics::OwnerExists, Repo, Index, Body> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateIssueCommentOptionPostBuilder<Owner, crate::generics::RepoExists, Index, Body> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> CreateIssueCommentOptionPostBuilder<Owner, Repo, crate::generics::IndexExists, Body> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> CreateIssueCommentOptionPostBuilder<Owner, Repo, Index, crate::generics::BodyExists> {
        self.inner.body.body = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateIssueCommentOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::BodyExists> {
    type Output = crate::comment::Comment;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/comments", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::comment::Comment, CreateIssueCommentOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::BodyExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
