
/// MergePullRequestForm form for merging Pull Request
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MergePullRequestOption {
    #[serde(rename = "Do")]
    pub do_: crate::merge_pull_request_option::MergePullRequestOptionDo,
    #[serde(rename = "MergeMessageField")]
    pub merge_message_field: Option<String>,
    #[serde(rename = "MergeTitleField")]
    pub merge_title_field: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MergePullRequestOptionDo {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "rebase")]
    Rebase,
    #[serde(rename = "rebase-merge")]
    RebaseMerge,
    #[serde(rename = "squash")]
    Squash,
}
impl Default for MergePullRequestOptionDo {
    fn default() -> Self {
        MergePullRequestOptionDo::Merge
    }
}

impl MergePullRequestOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> MergePullRequestOptionBuilder<crate::generics::MissingDo> {
        MergePullRequestOptionBuilder {
            body: Default::default(),
            _do: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_merge_pull_request() -> MergePullRequestOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex, crate::generics::MissingDo> {
        MergePullRequestOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
            _do: core::marker::PhantomData,
        }
    }
}

impl Into<MergePullRequestOption> for MergePullRequestOptionBuilder<crate::generics::DoExists> {
    fn into(self) -> MergePullRequestOption {
        self.body
    }
}

impl Into<MergePullRequestOption> for MergePullRequestOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::DoExists> {
    fn into(self) -> MergePullRequestOption {
        self.inner.body
    }
}

/// Builder for [`MergePullRequestOption`](./struct.MergePullRequestOption.html) object.
#[derive(Debug, Clone)]
pub struct MergePullRequestOptionBuilder<Do> {
    body: self::MergePullRequestOption,
    _do: core::marker::PhantomData<Do>,
}

impl<Do> MergePullRequestOptionBuilder<Do> {
    #[inline]
    pub fn do_(mut self, value: crate::merge_pull_request_option::MergePullRequestOptionDo) -> MergePullRequestOptionBuilder<crate::generics::DoExists> {
        self.body.do_ = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn merge_message_field(mut self, value: impl Into<String>) -> Self {
        self.body.merge_message_field = Some(value.into());
        self
    }

    #[inline]
    pub fn merge_title_field(mut self, value: impl Into<String>) -> Self {
        self.body.merge_title_field = Some(value.into());
        self
    }
}

/// Builder created by [`MergePullRequestOption::repo_merge_pull_request`](./struct.MergePullRequestOption.html#method.repo_merge_pull_request) method for a `POST` operation associated with `MergePullRequestOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MergePullRequestOptionPostBuilder<Owner, Repo, Index, Do> {
    inner: MergePullRequestOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
    _do: core::marker::PhantomData<Do>,
}

#[derive(Debug, Default, Clone)]
struct MergePullRequestOptionPostBuilderContainer {
    body: self::MergePullRequestOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index, Do> MergePullRequestOptionPostBuilder<Owner, Repo, Index, Do> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> MergePullRequestOptionPostBuilder<crate::generics::OwnerExists, Repo, Index, Do> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> MergePullRequestOptionPostBuilder<Owner, crate::generics::RepoExists, Index, Do> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the pull request to merge
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> MergePullRequestOptionPostBuilder<Owner, Repo, crate::generics::IndexExists, Do> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn do_(mut self, value: crate::merge_pull_request_option::MergePullRequestOptionDo) -> MergePullRequestOptionPostBuilder<Owner, Repo, Index, crate::generics::DoExists> {
        self.inner.body.do_ = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn merge_message_field(mut self, value: impl Into<String>) -> Self {
        self.inner.body.merge_message_field = Some(value.into());
        self
    }

    #[inline]
    pub fn merge_title_field(mut self, value: impl Into<String>) -> Self {
        self.inner.body.merge_title_field = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MergePullRequestOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::DoExists> {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/pulls/{index}/merge", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}

impl crate::client::ResponseWrapper<serde_json::Value, MergePullRequestOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists, crate::generics::DoExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

