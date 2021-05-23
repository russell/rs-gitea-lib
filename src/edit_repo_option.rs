
/// EditRepoOption options when editing a repository's properties
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditRepoOption {
    /// either `true` to allow mark pr as merged manually, or `false` to prevent it. `has_pull_requests` must be `true`.
    pub allow_manual_merge: Option<bool>,
    /// either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits. `has_pull_requests` must be `true`.
    pub allow_merge_commits: Option<bool>,
    /// either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging. `has_pull_requests` must be `true`.
    pub allow_rebase: Option<bool>,
    /// either `true` to allow rebase with explicit merge commits (--no-ff), or `false` to prevent rebase with explicit merge commits. `has_pull_requests` must be `true`.
    pub allow_rebase_explicit: Option<bool>,
    /// either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging. `has_pull_requests` must be `true`.
    pub allow_squash_merge: Option<bool>,
    /// set to `true` to archive this repository.
    pub archived: Option<bool>,
    /// either `true` to enable AutodetectManualMerge, or `false` to prevent it. `has_pull_requests` must be `true`, Note: In some special cases, misjudgments can occur.
    pub autodetect_manual_merge: Option<bool>,
    /// sets the default branch for this repository.
    pub default_branch: Option<String>,
    /// a short description of the repository.
    pub description: Option<String>,
    pub external_tracker: Option<crate::external_tracker::ExternalTracker>,
    pub external_wiki: Option<crate::external_wiki::ExternalWiki>,
    /// either `true` to enable issues for this repository or `false` to disable them.
    pub has_issues: Option<bool>,
    /// either `true` to enable project unit, or `false` to disable them.
    pub has_projects: Option<bool>,
    /// either `true` to allow pull requests, or `false` to prevent pull request.
    pub has_pull_requests: Option<bool>,
    /// either `true` to enable the wiki for this repository or `false` to disable it.
    pub has_wiki: Option<bool>,
    /// either `true` to ignore whitespace for conflicts, or `false` to not ignore whitespace. `has_pull_requests` must be `true`.
    pub ignore_whitespace_conflicts: Option<bool>,
    pub internal_tracker: Option<crate::internal_tracker::InternalTracker>,
    /// set to a string like `8h30m0s` to set the mirror interval time
    pub mirror_interval: Option<String>,
    /// name of the repository
    pub name: Option<String>,
    /// either `true` to make the repository private or `false` to make it public.
    /// Note: you will get a 422 error if the organization restricts changing repository visibility to organization
    /// owners and a non-owner tries to change the value of private.
    pub private: Option<bool>,
    /// either `true` to make this repository a template or `false` to make it a normal repository
    pub template: Option<bool>,
    /// a URL with more information about the repository.
    pub website: Option<String>,
}

impl EditRepoOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> EditRepoOptionBuilder {
        EditRepoOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_edit() -> EditRepoOptionPatchBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        EditRepoOptionPatchBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<EditRepoOption> for EditRepoOptionBuilder {
    fn into(self) -> EditRepoOption {
        self.body
    }
}

impl Into<EditRepoOption> for EditRepoOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    fn into(self) -> EditRepoOption {
        self.inner.body
    }
}

/// Builder for [`EditRepoOption`](./struct.EditRepoOption.html) object.
#[derive(Debug, Clone)]
pub struct EditRepoOptionBuilder {
    body: self::EditRepoOption,
}

impl EditRepoOptionBuilder {
    /// either `true` to allow mark pr as merged manually, or `false` to prevent it. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_manual_merge(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_manual_merge = Some(value.into());
        self
    }

    /// either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_merge_commits(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_merge_commits = Some(value.into());
        self
    }

    /// either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_rebase(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_rebase = Some(value.into());
        self
    }

    /// either `true` to allow rebase with explicit merge commits (--no-ff), or `false` to prevent rebase with explicit merge commits. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_rebase_explicit(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_rebase_explicit = Some(value.into());
        self
    }

    /// either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_squash_merge(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_squash_merge = Some(value.into());
        self
    }

    /// set to `true` to archive this repository.
    #[inline]
    pub fn archived(mut self, value: impl Into<bool>) -> Self {
        self.body.archived = Some(value.into());
        self
    }

    /// either `true` to enable AutodetectManualMerge, or `false` to prevent it. `has_pull_requests` must be `true`, Note: In some special cases, misjudgments can occur.
    #[inline]
    pub fn autodetect_manual_merge(mut self, value: impl Into<bool>) -> Self {
        self.body.autodetect_manual_merge = Some(value.into());
        self
    }

    /// sets the default branch for this repository.
    #[inline]
    pub fn default_branch(mut self, value: impl Into<String>) -> Self {
        self.body.default_branch = Some(value.into());
        self
    }

    /// a short description of the repository.
    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn external_tracker(mut self, value: crate::external_tracker::ExternalTracker) -> Self {
        self.body.external_tracker = Some(value.into());
        self
    }

    #[inline]
    pub fn external_wiki(mut self, value: crate::external_wiki::ExternalWiki) -> Self {
        self.body.external_wiki = Some(value.into());
        self
    }

    /// either `true` to enable issues for this repository or `false` to disable them.
    #[inline]
    pub fn has_issues(mut self, value: impl Into<bool>) -> Self {
        self.body.has_issues = Some(value.into());
        self
    }

    /// either `true` to enable project unit, or `false` to disable them.
    #[inline]
    pub fn has_projects(mut self, value: impl Into<bool>) -> Self {
        self.body.has_projects = Some(value.into());
        self
    }

    /// either `true` to allow pull requests, or `false` to prevent pull request.
    #[inline]
    pub fn has_pull_requests(mut self, value: impl Into<bool>) -> Self {
        self.body.has_pull_requests = Some(value.into());
        self
    }

    /// either `true` to enable the wiki for this repository or `false` to disable it.
    #[inline]
    pub fn has_wiki(mut self, value: impl Into<bool>) -> Self {
        self.body.has_wiki = Some(value.into());
        self
    }

    /// either `true` to ignore whitespace for conflicts, or `false` to not ignore whitespace. `has_pull_requests` must be `true`.
    #[inline]
    pub fn ignore_whitespace_conflicts(mut self, value: impl Into<bool>) -> Self {
        self.body.ignore_whitespace_conflicts = Some(value.into());
        self
    }

    #[inline]
    pub fn internal_tracker(mut self, value: crate::internal_tracker::InternalTracker) -> Self {
        self.body.internal_tracker = Some(value.into());
        self
    }

    /// set to a string like `8h30m0s` to set the mirror interval time
    #[inline]
    pub fn mirror_interval(mut self, value: impl Into<String>) -> Self {
        self.body.mirror_interval = Some(value.into());
        self
    }

    /// name of the repository
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    /// either `true` to make the repository private or `false` to make it public.
    /// Note: you will get a 422 error if the organization restricts changing repository visibility to organization
    /// owners and a non-owner tries to change the value of private.
    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.body.private = Some(value.into());
        self
    }

    /// either `true` to make this repository a template or `false` to make it a normal repository
    #[inline]
    pub fn template(mut self, value: impl Into<bool>) -> Self {
        self.body.template = Some(value.into());
        self
    }

    /// a URL with more information about the repository.
    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.body.website = Some(value.into());
        self
    }
}

/// Builder created by [`EditRepoOption::repo_edit`](./struct.EditRepoOption.html#method.repo_edit) method for a `PATCH` operation associated with `EditRepoOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EditRepoOptionPatchBuilder<Owner, Repo> {
    inner: EditRepoOptionPatchBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct EditRepoOptionPatchBuilderContainer {
    body: self::EditRepoOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> EditRepoOptionPatchBuilder<Owner, Repo> {
    /// owner of the repo to edit
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> EditRepoOptionPatchBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo to edit
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> EditRepoOptionPatchBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// either `true` to allow mark pr as merged manually, or `false` to prevent it. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_manual_merge(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.allow_manual_merge = Some(value.into());
        self
    }

    /// either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_merge_commits(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.allow_merge_commits = Some(value.into());
        self
    }

    /// either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_rebase(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.allow_rebase = Some(value.into());
        self
    }

    /// either `true` to allow rebase with explicit merge commits (--no-ff), or `false` to prevent rebase with explicit merge commits. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_rebase_explicit(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.allow_rebase_explicit = Some(value.into());
        self
    }

    /// either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging. `has_pull_requests` must be `true`.
    #[inline]
    pub fn allow_squash_merge(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.allow_squash_merge = Some(value.into());
        self
    }

    /// set to `true` to archive this repository.
    #[inline]
    pub fn archived(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.archived = Some(value.into());
        self
    }

    /// either `true` to enable AutodetectManualMerge, or `false` to prevent it. `has_pull_requests` must be `true`, Note: In some special cases, misjudgments can occur.
    #[inline]
    pub fn autodetect_manual_merge(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.autodetect_manual_merge = Some(value.into());
        self
    }

    /// sets the default branch for this repository.
    #[inline]
    pub fn default_branch(mut self, value: impl Into<String>) -> Self {
        self.inner.body.default_branch = Some(value.into());
        self
    }

    /// a short description of the repository.
    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn external_tracker(mut self, value: crate::external_tracker::ExternalTracker) -> Self {
        self.inner.body.external_tracker = Some(value.into());
        self
    }

    #[inline]
    pub fn external_wiki(mut self, value: crate::external_wiki::ExternalWiki) -> Self {
        self.inner.body.external_wiki = Some(value.into());
        self
    }

    /// either `true` to enable issues for this repository or `false` to disable them.
    #[inline]
    pub fn has_issues(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.has_issues = Some(value.into());
        self
    }

    /// either `true` to enable project unit, or `false` to disable them.
    #[inline]
    pub fn has_projects(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.has_projects = Some(value.into());
        self
    }

    /// either `true` to allow pull requests, or `false` to prevent pull request.
    #[inline]
    pub fn has_pull_requests(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.has_pull_requests = Some(value.into());
        self
    }

    /// either `true` to enable the wiki for this repository or `false` to disable it.
    #[inline]
    pub fn has_wiki(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.has_wiki = Some(value.into());
        self
    }

    /// either `true` to ignore whitespace for conflicts, or `false` to not ignore whitespace. `has_pull_requests` must be `true`.
    #[inline]
    pub fn ignore_whitespace_conflicts(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.ignore_whitespace_conflicts = Some(value.into());
        self
    }

    #[inline]
    pub fn internal_tracker(mut self, value: crate::internal_tracker::InternalTracker) -> Self {
        self.inner.body.internal_tracker = Some(value.into());
        self
    }

    /// set to a string like `8h30m0s` to set the mirror interval time
    #[inline]
    pub fn mirror_interval(mut self, value: impl Into<String>) -> Self {
        self.inner.body.mirror_interval = Some(value.into());
        self
    }

    /// name of the repository
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.inner.body.name = Some(value.into());
        self
    }

    /// either `true` to make the repository private or `false` to make it public.
    /// Note: you will get a 422 error if the organization restricts changing repository visibility to organization
    /// owners and a non-owner tries to change the value of private.
    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.private = Some(value.into());
        self
    }

    /// either `true` to make this repository a template or `false` to make it a normal repository
    #[inline]
    pub fn template(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.template = Some(value.into());
        self
    }

    /// a URL with more information about the repository.
    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.inner.body.website = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for EditRepoOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = crate::repository::Repository;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::repository::Repository, EditRepoOptionPatchBuilder<crate::generics::OwnerExists, crate::generics::RepoExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
