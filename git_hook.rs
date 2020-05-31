
/// GitHook represents a Git repository hook
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitHook {
    pub content: Option<String>,
    pub is_active: Option<bool>,
    pub name: Option<String>,
}

impl GitHook {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GitHookBuilder {
        GitHookBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_git_hooks() -> GitHookGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        GitHookGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_git_hook() -> GitHookGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        GitHookGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<GitHook> for GitHookBuilder {
    fn into(self) -> GitHook {
        self.body
    }
}

/// Builder for [`GitHook`](./struct.GitHook.html) object.
#[derive(Debug, Clone)]
pub struct GitHookBuilder {
    body: self::GitHook,
}

impl GitHookBuilder {
    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.body.content = Some(value.into());
        self
    }

    #[inline]
    pub fn is_active(mut self, value: impl Into<bool>) -> Self {
        self.body.is_active = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }
}

/// Builder created by [`GitHook::repo_list_git_hooks`](./struct.GitHook.html#method.repo_list_git_hooks) method for a `GET` operation associated with `GitHook`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GitHookGetBuilder<Owner, Repo> {
    inner: GitHookGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct GitHookGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> GitHookGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> GitHookGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> GitHookGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GitHookGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<GitHook>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks/git", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`GitHook::repo_get_git_hook`](./struct.GitHook.html#method.repo_get_git_hook) method for a `GET` operation associated with `GitHook`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GitHookGetBuilder1<Owner, Repo, Id> {
    inner: GitHookGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct GitHookGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<String>,
}

impl<Owner, Repo, Id> GitHookGetBuilder1<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> GitHookGetBuilder1<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> GitHookGetBuilder1<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the hook to get
    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> GitHookGetBuilder1<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GitHookGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = GitHook;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks/git/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}
