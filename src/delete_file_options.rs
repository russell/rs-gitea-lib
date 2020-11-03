
/// DeleteFileOptions options for deleting files (used for other File structs below)
/// Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteFileOptions {
    pub author: Option<crate::identity::Identity>,
    /// branch (optional) to base this file from. if not given, the default branch is used
    pub branch: Option<String>,
    pub committer: Option<crate::identity::Identity>,
    pub dates: Option<crate::commit_date_options::CommitDateOptions>,
    /// message (optional) for the commit of this file. if not supplied, a default message will be used
    pub message: Option<String>,
    /// new_branch (optional) will make a new branch from `branch` before creating the file
    pub new_branch: Option<String>,
    /// sha is the SHA for the file that already exists
    pub sha: String,
}

impl DeleteFileOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> DeleteFileOptionsBuilder<crate::generics::MissingSha> {
        DeleteFileOptionsBuilder {
            body: Default::default(),
            _sha: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_delete_file() -> DeleteFileOptionsDeleteBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingFilepath, crate::generics::MissingSha> {
        DeleteFileOptionsDeleteBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_filepath: core::marker::PhantomData,
            _sha: core::marker::PhantomData,
        }
    }
}

impl Into<DeleteFileOptions> for DeleteFileOptionsBuilder<crate::generics::ShaExists> {
    fn into(self) -> DeleteFileOptions {
        self.body
    }
}

impl Into<DeleteFileOptions> for DeleteFileOptionsDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::FilepathExists, crate::generics::ShaExists> {
    fn into(self) -> DeleteFileOptions {
        self.inner.body
    }
}

/// Builder for [`DeleteFileOptions`](./struct.DeleteFileOptions.html) object.
#[derive(Debug, Clone)]
pub struct DeleteFileOptionsBuilder<Sha> {
    body: self::DeleteFileOptions,
    _sha: core::marker::PhantomData<Sha>,
}

impl<Sha> DeleteFileOptionsBuilder<Sha> {
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

    #[inline]
    pub fn dates(mut self, value: crate::commit_date_options::CommitDateOptions) -> Self {
        self.body.dates = Some(value.into());
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
    pub fn sha(mut self, value: impl Into<String>) -> DeleteFileOptionsBuilder<crate::generics::ShaExists> {
        self.body.sha = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`DeleteFileOptions::repo_delete_file`](./struct.DeleteFileOptions.html#method.repo_delete_file) method for a `DELETE` operation associated with `DeleteFileOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct DeleteFileOptionsDeleteBuilder<Owner, Repo, Filepath, Sha> {
    inner: DeleteFileOptionsDeleteBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_filepath: core::marker::PhantomData<Filepath>,
    _sha: core::marker::PhantomData<Sha>,
}

#[derive(Debug, Default, Clone)]
struct DeleteFileOptionsDeleteBuilderContainer {
    body: self::DeleteFileOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_filepath: Option<String>,
}

impl<Owner, Repo, Filepath, Sha> DeleteFileOptionsDeleteBuilder<Owner, Repo, Filepath, Sha> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> DeleteFileOptionsDeleteBuilder<crate::generics::OwnerExists, Repo, Filepath, Sha> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> DeleteFileOptionsDeleteBuilder<Owner, crate::generics::RepoExists, Filepath, Sha> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// path of the file to delete
    #[inline]
    pub fn filepath(mut self, value: impl Into<String>) -> DeleteFileOptionsDeleteBuilder<Owner, Repo, crate::generics::FilepathExists, Sha> {
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

    #[inline]
    pub fn dates(mut self, value: crate::commit_date_options::CommitDateOptions) -> Self {
        self.inner.body.dates = Some(value.into());
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
    pub fn sha(mut self, value: impl Into<String>) -> DeleteFileOptionsDeleteBuilder<Owner, Repo, Filepath, crate::generics::ShaExists> {
        self.inner.body.sha = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for DeleteFileOptionsDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::FilepathExists, crate::generics::ShaExists> {
    type Output = crate::file_delete_response::FileDeleteResponse;

    const METHOD: http::Method = http::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/contents/{filepath}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), filepath=self.inner.param_filepath.as_ref().expect("missing parameter filepath?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

impl crate::client::ResponseWrapper<crate::file_delete_response::FileDeleteResponse, DeleteFileOptionsDeleteBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::FilepathExists, crate::generics::ShaExists>> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}
