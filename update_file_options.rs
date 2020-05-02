
/// UpdateFileOptions options for updating files
/// Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateFileOptions {
    pub author: Option<crate::identity::Identity>,
    /// branch (optional) to base this file from. if not given, the default branch is used
    pub branch: Option<String>,
    pub committer: Option<crate::identity::Identity>,
    /// content must be base64 encoded
    pub content: String,
    pub dates: Option<crate::commit_date_options::CommitDateOptions>,
    /// from_path (optional) is the path of the original file which will be moved/renamed to the path in the URL
    pub from_path: Option<String>,
    /// message (optional) for the commit of this file. if not supplied, a default message will be used
    pub message: Option<String>,
    /// new_branch (optional) will make a new branch from `branch` before creating the file
    pub new_branch: Option<String>,
    /// sha is the SHA for the file that already exists
    pub sha: String,
}

impl UpdateFileOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> UpdateFileOptionsBuilder<crate::generics::MissingContent, crate::generics::MissingSha> {
        UpdateFileOptionsBuilder {
            body: Default::default(),
            _content: core::marker::PhantomData,
            _sha: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_update_file() -> UpdateFileOptionsPutBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingFilepath, crate::generics::MissingContent, crate::generics::MissingSha> {
        UpdateFileOptionsPutBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_filepath: core::marker::PhantomData,
            _content: core::marker::PhantomData,
            _sha: core::marker::PhantomData,
        }
    }
}

impl Into<UpdateFileOptions> for UpdateFileOptionsBuilder<crate::generics::ContentExists, crate::generics::ShaExists> {
    fn into(self) -> UpdateFileOptions {
        self.body
    }
}

impl Into<UpdateFileOptions> for UpdateFileOptionsPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::FilepathExists, crate::generics::ContentExists, crate::generics::ShaExists> {
    fn into(self) -> UpdateFileOptions {
        self.inner.body
    }
}

/// Builder for [`UpdateFileOptions`](./struct.UpdateFileOptions.html) object.
#[derive(Debug, Clone)]
pub struct UpdateFileOptionsBuilder<Content, Sha> {
    body: self::UpdateFileOptions,
    _content: core::marker::PhantomData<Content>,
    _sha: core::marker::PhantomData<Sha>,
}

impl<Content, Sha> UpdateFileOptionsBuilder<Content, Sha> {
    #[inline]
    pub fn author(mut self, value: crate::identity::Identity) -> Self {
        self.body.author = Some(value.into());
        self
    }

    /// branch (optional) to base this file from. if not given, the default branch is used
    #[inline]
    pub fn branch(mut self, value: impl Into<String>) -> Self {
        self.body.branch = Some(value.into());
        self
    }

    #[inline]
    pub fn committer(mut self, value: crate::identity::Identity) -> Self {
        self.body.committer = Some(value.into());
        self
    }

    /// content must be base64 encoded
    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> UpdateFileOptionsBuilder<crate::generics::ContentExists, Sha> {
        self.body.content = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn dates(mut self, value: crate::commit_date_options::CommitDateOptions) -> Self {
        self.body.dates = Some(value.into());
        self
    }

    /// from_path (optional) is the path of the original file which will be moved/renamed to the path in the URL
    #[inline]
    pub fn from_path(mut self, value: impl Into<String>) -> Self {
        self.body.from_path = Some(value.into());
        self
    }

    /// message (optional) for the commit of this file. if not supplied, a default message will be used
    #[inline]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.body.message = Some(value.into());
        self
    }

    /// new_branch (optional) will make a new branch from `branch` before creating the file
    #[inline]
    pub fn new_branch(mut self, value: impl Into<String>) -> Self {
        self.body.new_branch = Some(value.into());
        self
    }

    /// sha is the SHA for the file that already exists
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> UpdateFileOptionsBuilder<Content, crate::generics::ShaExists> {
        self.body.sha = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`UpdateFileOptions::repo_update_file`](./struct.UpdateFileOptions.html#method.repo_update_file) method for a `PUT` operation associated with `UpdateFileOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UpdateFileOptionsPutBuilder<Owner, Repo, Filepath, Content, Sha> {
    inner: UpdateFileOptionsPutBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_filepath: core::marker::PhantomData<Filepath>,
    _content: core::marker::PhantomData<Content>,
    _sha: core::marker::PhantomData<Sha>,
}

#[derive(Debug, Default, Clone)]
struct UpdateFileOptionsPutBuilderContainer {
    body: self::UpdateFileOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_filepath: Option<String>,
}

impl<Owner, Repo, Filepath, Content, Sha> UpdateFileOptionsPutBuilder<Owner, Repo, Filepath, Content, Sha> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> UpdateFileOptionsPutBuilder<crate::generics::OwnerExists, Repo, Filepath, Content, Sha> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> UpdateFileOptionsPutBuilder<Owner, crate::generics::RepoExists, Filepath, Content, Sha> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// path of the file to update
    #[inline]
    pub fn filepath(mut self, value: impl Into<String>) -> UpdateFileOptionsPutBuilder<Owner, Repo, crate::generics::FilepathExists, Content, Sha> {
        self.inner.param_filepath = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn author(mut self, value: crate::identity::Identity) -> Self {
        self.inner.body.author = Some(value.into());
        self
    }

    /// branch (optional) to base this file from. if not given, the default branch is used
    #[inline]
    pub fn branch(mut self, value: impl Into<String>) -> Self {
        self.inner.body.branch = Some(value.into());
        self
    }

    #[inline]
    pub fn committer(mut self, value: crate::identity::Identity) -> Self {
        self.inner.body.committer = Some(value.into());
        self
    }

    /// content must be base64 encoded
    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> UpdateFileOptionsPutBuilder<Owner, Repo, Filepath, crate::generics::ContentExists, Sha> {
        self.inner.body.content = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn dates(mut self, value: crate::commit_date_options::CommitDateOptions) -> Self {
        self.inner.body.dates = Some(value.into());
        self
    }

    /// from_path (optional) is the path of the original file which will be moved/renamed to the path in the URL
    #[inline]
    pub fn from_path(mut self, value: impl Into<String>) -> Self {
        self.inner.body.from_path = Some(value.into());
        self
    }

    /// message (optional) for the commit of this file. if not supplied, a default message will be used
    #[inline]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.inner.body.message = Some(value.into());
        self
    }

    /// new_branch (optional) will make a new branch from `branch` before creating the file
    #[inline]
    pub fn new_branch(mut self, value: impl Into<String>) -> Self {
        self.inner.body.new_branch = Some(value.into());
        self
    }

    /// sha is the SHA for the file that already exists
    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> UpdateFileOptionsPutBuilder<Owner, Repo, Filepath, Content, crate::generics::ShaExists> {
        self.inner.body.sha = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UpdateFileOptionsPutBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::FilepathExists, crate::generics::ContentExists, crate::generics::ShaExists> {
    type Output = crate::post_repos_owner_repo_contents_filepath_response::PostReposOwnerRepoContentsFilepathResponse;

    const METHOD: http::Method = http::Method::PUT;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/contents/{filepath}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), filepath=self.inner.param_filepath.as_ref().expect("missing parameter filepath?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
