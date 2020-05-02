
/// CreateFileOptions options for creating files
/// Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateFileOptions {
    pub author: Option<crate::identity::Identity>,
    /// branch (optional) to base this file from. if not given, the default branch is used
    pub branch: Option<String>,
    pub committer: Option<crate::identity::Identity>,
    /// content must be base64 encoded
    pub content: String,
    pub dates: Option<crate::commit_date_options::CommitDateOptions>,
    /// message (optional) for the commit of this file. if not supplied, a default message will be used
    pub message: Option<String>,
    /// new_branch (optional) will make a new branch from `branch` before creating the file
    pub new_branch: Option<String>,
}

impl CreateFileOptions {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateFileOptionsBuilder<crate::generics::MissingContent> {
        CreateFileOptionsBuilder {
            body: Default::default(),
            _content: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_create_file() -> CreateFileOptionsPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingFilepath, crate::generics::MissingContent> {
        CreateFileOptionsPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_filepath: core::marker::PhantomData,
            _content: core::marker::PhantomData,
        }
    }
}

impl Into<CreateFileOptions> for CreateFileOptionsBuilder<crate::generics::ContentExists> {
    fn into(self) -> CreateFileOptions {
        self.body
    }
}

impl Into<CreateFileOptions> for CreateFileOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::FilepathExists, crate::generics::ContentExists> {
    fn into(self) -> CreateFileOptions {
        self.inner.body
    }
}

/// Builder for [`CreateFileOptions`](./struct.CreateFileOptions.html) object.
#[derive(Debug, Clone)]
pub struct CreateFileOptionsBuilder<Content> {
    body: self::CreateFileOptions,
    _content: core::marker::PhantomData<Content>,
}

impl<Content> CreateFileOptionsBuilder<Content> {
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
    pub fn content(mut self, value: impl Into<String>) -> CreateFileOptionsBuilder<crate::generics::ContentExists> {
        self.body.content = value.into();
        unsafe { std::mem::transmute(self) }
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
}

/// Builder created by [`CreateFileOptions::repo_create_file`](./struct.CreateFileOptions.html#method.repo_create_file) method for a `POST` operation associated with `CreateFileOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateFileOptionsPostBuilder<Owner, Repo, Filepath, Content> {
    inner: CreateFileOptionsPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_filepath: core::marker::PhantomData<Filepath>,
    _content: core::marker::PhantomData<Content>,
}

#[derive(Debug, Default, Clone)]
struct CreateFileOptionsPostBuilderContainer {
    body: self::CreateFileOptions,
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_filepath: Option<String>,
}

impl<Owner, Repo, Filepath, Content> CreateFileOptionsPostBuilder<Owner, Repo, Filepath, Content> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateFileOptionsPostBuilder<crate::generics::OwnerExists, Repo, Filepath, Content> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateFileOptionsPostBuilder<Owner, crate::generics::RepoExists, Filepath, Content> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// path of the file to create
    #[inline]
    pub fn filepath(mut self, value: impl Into<String>) -> CreateFileOptionsPostBuilder<Owner, Repo, crate::generics::FilepathExists, Content> {
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
    pub fn content(mut self, value: impl Into<String>) -> CreateFileOptionsPostBuilder<Owner, Repo, Filepath, crate::generics::ContentExists> {
        self.inner.body.content = value.into();
        unsafe { std::mem::transmute(self) }
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
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateFileOptionsPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::FilepathExists, crate::generics::ContentExists> {
    type Output = crate::post_repos_owner_repo_contents_filepath_response::PostReposOwnerRepoContentsFilepathResponse;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/contents/{filepath}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), filepath=self.inner.param_filepath.as_ref().expect("missing parameter filepath?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
