
/// EditGitHookOption options when modifying one Git hook
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditGitHookOption {
    pub content: Option<String>,
}

impl EditGitHookOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditGitHookOptionBuilder {
        EditGitHookOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_edit_git_hook() -> EditGitHookOptionPatchBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingId> {
        EditGitHookOptionPatchBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<EditGitHookOption> for EditGitHookOptionBuilder {
    fn into(self) -> EditGitHookOption {
        self.body
    }
}

impl Into<EditGitHookOption> for EditGitHookOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    fn into(self) -> EditGitHookOption {
        self.inner.body
    }
}

/// Builder for [`EditGitHookOption`](./struct.EditGitHookOption.html) object.
#[derive(Debug, Clone)]
pub struct EditGitHookOptionBuilder {
    body: self::EditGitHookOption,
}

impl EditGitHookOptionBuilder {
    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.body.content = Some(value.into());
        self
    }
}

/// Builder created by [`EditGitHookOption::repo_edit_git_hook`](./struct.EditGitHookOption.html#method.repo_edit_git_hook) method for a `PATCH` operation associated with `EditGitHookOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditGitHookOptionPatchBuilder<Owner, Repo, Id> {
    inner: EditGitHookOptionPatchBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct EditGitHookOptionPatchBuilderContainer {
    body: self::EditGitHookOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_id: Option<String>,
}

impl<Owner, Repo, Id> EditGitHookOptionPatchBuilder<Owner, Repo, Id> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditGitHookOptionPatchBuilder<crate::generics::OwnerExists, Repo, Id> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditGitHookOptionPatchBuilder<Owner, crate::generics::RepoExists, Id> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// id of the hook to get
    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> EditGitHookOptionPatchBuilder<Owner, Repo, crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.inner.body.content = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditGitHookOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IdExists> {
    type Output = crate::patch_repos_owner_repo_hooks_git_id_response::PatchReposOwnerRepoHooksGitIdResponse;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/hooks/git/{id}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
