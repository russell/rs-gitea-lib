#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub author: Option<crate::user::User>,
    pub commit: Option<crate::repo_commit::RepoCommit>,
    pub committer: Option<crate::user::User>,
    pub html_url: Option<String>,
    pub parents: Option<Vec<crate::commit_meta::CommitMeta>>,
    pub sha: Option<String>,
    pub url: Option<String>,
}

impl Commit {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CommitBuilder {
        CommitBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_get_all_commits() -> CommitGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        CommitGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_single_commit() -> CommitGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingSha> {
        CommitGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_sha: core::marker::PhantomData,
        }
    }
}

impl Into<Commit> for CommitBuilder {
    fn into(self) -> Commit {
        self.body
    }
}

/// Builder for [`Commit`](./struct.Commit.html) object.
#[derive(Debug, Clone)]
pub struct CommitBuilder {
    body: self::Commit,
}

impl CommitBuilder {
    #[inline]
    pub fn author(mut self, value: crate::user::User) -> Self {
        self.body.author = Some(value.into());
        self
    }

    #[inline]
    pub fn commit(mut self, value: crate::repo_commit::RepoCommit) -> Self {
        self.body.commit = Some(value.into());
        self
    }

    #[inline]
    pub fn committer(mut self, value: crate::user::User) -> Self {
        self.body.committer = Some(value.into());
        self
    }

    #[inline]
    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.body.html_url = Some(value.into());
        self
    }

    #[inline]
    pub fn parents(mut self, value: impl Iterator<Item = crate::commit_meta::CommitMeta>) -> Self {
        self.body.parents = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}

/// Builder created by [`Commit::repo_get_all_commits`](./struct.Commit.html#method.repo_get_all_commits) method for a `GET` operation associated with `Commit`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommitGetBuilder<Owner, Repo> {
    inner: CommitGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct CommitGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_sha: Option<String>,
    param_page: Option<i64>,
}

impl<Owner, Repo> CommitGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CommitGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CommitGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// SHA or branch to start listing commits from (usually 'master')
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.inner.param_sha = Some(value.into());
        self
    }

    /// page number of requested commits
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_page = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CommitGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Commit>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/commits", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("sha", self.inner.param_sha.as_ref().map(std::string::ToString::to_string)),
            ("page", self.inner.param_page.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Commit::repo_get_single_commit`](./struct.Commit.html#method.repo_get_single_commit) method for a `GET` operation associated with `Commit`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommitGetBuilder1<Owner, Repo, Sha> {
    inner: CommitGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_sha: core::marker::PhantomData<Sha>,
}

#[derive(Debug, Default, Clone)]
struct CommitGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_sha: Option<String>,
}

impl<Owner, Repo, Sha> CommitGetBuilder1<Owner, Repo, Sha> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CommitGetBuilder1<crate::generics::OwnerExists, Repo, Sha> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CommitGetBuilder1<Owner, crate::generics::RepoExists, Sha> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// the commit hash
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> CommitGetBuilder1<Owner, Repo, crate::generics::ShaExists> {
        self.inner.param_sha = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CommitGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ShaExists> {
    type Output = Commit;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/git/commits/{sha}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), sha=self.inner.param_sha.as_ref().expect("missing parameter sha?")).into()
    }
}
