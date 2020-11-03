
/// TransferRepoOption options when transfer a repository's ownership
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransferRepoOption {
    pub new_owner: String,
    /// ID of the team or teams to add to the repository. Teams can only be added to organization-owned repositories.
    pub team_ids: Option<Vec<i64>>,
}

impl TransferRepoOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TransferRepoOptionBuilder<crate::generics::MissingNewOwner> {
        TransferRepoOptionBuilder {
            body: Default::default(),
            _new_owner: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_transfer() -> TransferRepoOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingNewOwner> {
        TransferRepoOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _new_owner: core::marker::PhantomData,
        }
    }
}

impl Into<TransferRepoOption> for TransferRepoOptionBuilder<crate::generics::NewOwnerExists> {
    fn into(self) -> TransferRepoOption {
        self.body
    }
}

impl Into<TransferRepoOption> for TransferRepoOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NewOwnerExists> {
    fn into(self) -> TransferRepoOption {
        self.inner.body
    }
}

/// Builder for [`TransferRepoOption`](./struct.TransferRepoOption.html) object.
#[derive(Debug, Clone)]
pub struct TransferRepoOptionBuilder<NewOwner> {
    body: self::TransferRepoOption,
    _new_owner: core::marker::PhantomData<NewOwner>,
}

impl<NewOwner> TransferRepoOptionBuilder<NewOwner> {
    #[inline]
    pub fn new_owner(mut self, value: impl Into<String>) -> TransferRepoOptionBuilder<crate::generics::NewOwnerExists> {
        self.body.new_owner = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// ID of the team or teams to add to the repository. Teams can only be added to organization-owned repositories.
    #[inline]
    pub fn team_ids(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.body.team_ids = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`TransferRepoOption::repo_transfer`](./struct.TransferRepoOption.html#method.repo_transfer) method for a `POST` operation associated with `TransferRepoOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TransferRepoOptionPostBuilder<Owner, Repo, NewOwner> {
    inner: TransferRepoOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _new_owner: core::marker::PhantomData<NewOwner>,
}

#[derive(Debug, Default, Clone)]
struct TransferRepoOptionPostBuilderContainer {
    body: self::TransferRepoOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo, NewOwner> TransferRepoOptionPostBuilder<Owner, Repo, NewOwner> {
    /// owner of the repo to transfer
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> TransferRepoOptionPostBuilder<crate::generics::OwnerExists, Repo, NewOwner> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo to transfer
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> TransferRepoOptionPostBuilder<Owner, crate::generics::RepoExists, NewOwner> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn new_owner(mut self, value: impl Into<String>) -> TransferRepoOptionPostBuilder<Owner, Repo, crate::generics::NewOwnerExists> {
        self.inner.body.new_owner = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// ID of the team or teams to add to the repository. Teams can only be added to organization-owned repositories.
    #[inline]
    pub fn team_ids(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
        self.inner.body.team_ids = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for TransferRepoOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NewOwnerExists> {
    type Output = crate::repository::Repository;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/transfer", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::repository::Repository, TransferRepoOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::NewOwnerExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
