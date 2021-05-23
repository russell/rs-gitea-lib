
/// Branch represents a repository branch
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub commit: Option<crate::payload_commit::PayloadCommit>,
    pub effective_branch_protection_name: Option<String>,
    pub enable_status_check: Option<bool>,
    pub name: Option<String>,
    pub protected: Option<bool>,
    pub required_approvals: Option<i64>,
    pub status_check_contexts: Option<Vec<String>>,
    pub user_can_merge: Option<bool>,
    pub user_can_push: Option<bool>,
}

impl Branch {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> BranchBuilder {
        BranchBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_list_branches() -> BranchGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        BranchGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_branch() -> BranchGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingBranch> {
        BranchGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_branch: core::marker::PhantomData,
        }
    }
}

impl Into<Branch> for BranchBuilder {
    fn into(self) -> Branch {
        self.body
    }
}

/// Builder for [`Branch`](./struct.Branch.html) object.
#[derive(Debug, Clone)]
pub struct BranchBuilder {
    body: self::Branch,
}

impl BranchBuilder {
    #[inline]
    pub fn commit(mut self, value: crate::payload_commit::PayloadCommit) -> Self {
        self.body.commit = Some(value.into());
        self
    }

    #[inline]
    pub fn effective_branch_protection_name(mut self, value: impl Into<String>) -> Self {
        self.body.effective_branch_protection_name = Some(value.into());
        self
    }

    #[inline]
    pub fn enable_status_check(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_status_check = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn protected(mut self, value: impl Into<bool>) -> Self {
        self.body.protected = Some(value.into());
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
    pub fn user_can_merge(mut self, value: impl Into<bool>) -> Self {
        self.body.user_can_merge = Some(value.into());
        self
    }

    #[inline]
    pub fn user_can_push(mut self, value: impl Into<bool>) -> Self {
        self.body.user_can_push = Some(value.into());
        self
    }
}

/// Builder created by [`Branch::repo_list_branches`](./struct.Branch.html#method.repo_list_branches) method for a `GET` operation associated with `Branch`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct BranchGetBuilder<Owner, Repo> {
    inner: BranchGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct BranchGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> BranchGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> BranchGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> BranchGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for BranchGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Branch>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/branches", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
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

/// Builder created by [`Branch::repo_get_branch`](./struct.Branch.html#method.repo_get_branch) method for a `GET` operation associated with `Branch`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct BranchGetBuilder1<Owner, Repo, Branch> {
    inner: BranchGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_branch: core::marker::PhantomData<Branch>,
}

#[derive(Debug, Default, Clone)]
struct BranchGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_branch: Option<String>,
}

impl<Owner, Repo, Branch> BranchGetBuilder1<Owner, Repo, Branch> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> BranchGetBuilder1<crate::generics::OwnerExists, Repo, Branch> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> BranchGetBuilder1<Owner, crate::generics::RepoExists, Branch> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// branch to get
    #[inline]
    pub fn branch(mut self, value: impl Into<String>) -> BranchGetBuilder1<Owner, Repo, crate::generics::BranchExists> {
        self.inner.param_branch = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for BranchGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::BranchExists> {
    type Output = Branch;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/branches/{branch}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), branch=self.inner.param_branch.as_ref().expect("missing parameter branch?")).into()
    }
}
