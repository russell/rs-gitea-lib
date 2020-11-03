#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetReposOwnerRepoLanguagesResponse {}

impl GetReposOwnerRepoLanguagesResponse {
    #[inline]
    pub fn repo_get_languages() -> GetReposOwnerRepoLanguagesResponseGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        GetReposOwnerRepoLanguagesResponseGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

/// Builder created by [`GetReposOwnerRepoLanguagesResponse::repo_get_languages`](./struct.GetReposOwnerRepoLanguagesResponse.html#method.repo_get_languages) method for a `GET` operation associated with `GetReposOwnerRepoLanguagesResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GetReposOwnerRepoLanguagesResponseGetBuilder<Owner, Repo> {
    inner: GetReposOwnerRepoLanguagesResponseGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct GetReposOwnerRepoLanguagesResponseGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> GetReposOwnerRepoLanguagesResponseGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> GetReposOwnerRepoLanguagesResponseGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> GetReposOwnerRepoLanguagesResponseGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetReposOwnerRepoLanguagesResponseGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = GetReposOwnerRepoLanguagesResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/languages", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}
