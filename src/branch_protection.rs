
/// BranchProtection represents a branch protection for a repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BranchProtection {
    pub approvals_whitelist_teams: Option<Vec<String>>,
    pub approvals_whitelist_username: Option<Vec<String>>,
    pub block_on_outdated_branch: Option<bool>,
    pub block_on_rejected_reviews: Option<bool>,
    pub branch_name: Option<String>,
    pub created_at: Option<String>,
    pub dismiss_stale_approvals: Option<bool>,
    pub enable_approvals_whitelist: Option<bool>,
    pub enable_merge_whitelist: Option<bool>,
    pub enable_push: Option<bool>,
    pub enable_push_whitelist: Option<bool>,
    pub enable_status_check: Option<bool>,
    pub merge_whitelist_teams: Option<Vec<String>>,
    pub merge_whitelist_usernames: Option<Vec<String>>,
    pub protected_file_patterns: Option<String>,
    pub push_whitelist_deploy_keys: Option<bool>,
    pub push_whitelist_teams: Option<Vec<String>>,
    pub push_whitelist_usernames: Option<Vec<String>>,
    pub require_signed_commits: Option<bool>,
    pub required_approvals: Option<i64>,
    pub status_check_contexts: Option<Vec<String>>,
    pub updated_at: Option<String>,
}

impl BranchProtection {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> BranchProtectionBuilder {
        BranchProtectionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_branch_protection() -> BranchProtectionGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        BranchProtectionGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_branch_protection() -> BranchProtectionGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingName> {
        BranchProtectionGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_name: core::marker::PhantomData,
        }
    }
}

impl Into<BranchProtection> for BranchProtectionBuilder {
    fn into(self) -> BranchProtection {
        self.body
    }
}

/// Builder for [`BranchProtection`](./struct.BranchProtection.html) object.
#[derive(Debug, Clone)]
pub struct BranchProtectionBuilder {
    body: self::BranchProtection,
}

impl BranchProtectionBuilder {
    #[inline]
    pub fn approvals_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.approvals_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn approvals_whitelist_username(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.approvals_whitelist_username = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn block_on_outdated_branch(mut self, value: impl Into<bool>) -> Self {
        self.body.block_on_outdated_branch = Some(value.into());
        self
    }

    #[inline]
    pub fn block_on_rejected_reviews(mut self, value: impl Into<bool>) -> Self {
        self.body.block_on_rejected_reviews = Some(value.into());
        self
    }

    #[inline]
    pub fn branch_name(mut self, value: impl Into<String>) -> Self {
        self.body.branch_name = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn dismiss_stale_approvals(mut self, value: impl Into<bool>) -> Self {
        self.body.dismiss_stale_approvals = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_approvals_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_approvals_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_merge_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_merge_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_push(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_push = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_push_whitelist(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_push_whitelist = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_status_check(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_status_check = Some(value.into());
        self
    }

    #[inline]
    pub fn merge_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.merge_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn merge_whitelist_usernames(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.merge_whitelist_usernames = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn protected_file_patterns(mut self, value: impl Into<String>) -> Self {
        self.body.protected_file_patterns = Some(value.into());
        self
    }

    #[inline]
    pub fn push_whitelist_deploy_keys(mut self, value: impl Into<bool>) -> Self {
        self.body.push_whitelist_deploy_keys = Some(value.into());
        self
    }

    #[inline]
    pub fn push_whitelist_teams(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.push_whitelist_teams = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn push_whitelist_usernames(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.push_whitelist_usernames = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn require_signed_commits(mut self, value: impl Into<bool>) -> Self {
        self.body.require_signed_commits = Some(value.into());
        self
    }

    #[inline]
    pub fn required_approvals(mut self, value: impl Into<i64>) -> Self {
        self.body.required_approvals = Some(value.into());
        self
    }

    #[inline]
    pub fn status_check_contexts(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.status_check_contexts = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }
}

/// Builder created by [`BranchProtection::repo_list_branch_protection`](./struct.BranchProtection.html#method.repo_list_branch_protection) method for a `GET` operation associated with `BranchProtection`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct BranchProtectionGetBuilder<Owner, Repo> {
    inner: BranchProtectionGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct BranchProtectionGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> BranchProtectionGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> BranchProtectionGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> BranchProtectionGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for BranchProtectionGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<BranchProtection>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/branch_protections", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`BranchProtection::repo_get_branch_protection`](./struct.BranchProtection.html#method.repo_get_branch_protection) method for a `GET` operation associated with `BranchProtection`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct BranchProtectionGetBuilder1<Owner, Repo, Name> {
    inner: BranchProtectionGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct BranchProtectionGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_name: Option<String>,
}

impl<Owner, Repo, Name> BranchProtectionGetBuilder1<Owner, Repo, Name> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> BranchProtectionGetBuilder1<crate::generics::OwnerExists, Repo, Name> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> BranchProtectionGetBuilder1<Owner, crate::generics::RepoExists, Name> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of protected branch
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> BranchProtectionGetBuilder1<Owner, Repo, crate::generics::NameExists> {
        self.inner.param_name = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for BranchProtectionGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NameExists> {
    type Output = BranchProtection;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/branch_protections/{name}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), name=self.inner.param_name.as_ref().expect("missing parameter name?")).into()
    }
}
