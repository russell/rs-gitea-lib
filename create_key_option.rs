
/// CreateKeyOption options when creating a key
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateKeyOption {
    /// An armored SSH key to add
    pub key: String,
    /// Describe if the key has only read access or read/write
    pub read_only: Option<bool>,
    /// Title of the key to add
    pub title: String,
}

impl CreateKeyOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateKeyOptionBuilder<crate::generics::MissingKey, crate::generics::MissingTitle> {
        CreateKeyOptionBuilder {
            body: Default::default(),
            _key: core::marker::PhantomData,
            _title: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn admin_create_public_key() -> CreateKeyOptionPostBuilder<crate::generics::MissingUsername, crate::generics::MissingKey, crate::generics::MissingTitle> {
        CreateKeyOptionPostBuilder {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
            _key: core::marker::PhantomData,
            _title: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_create_key() -> CreateKeyOptionPostBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingKey, crate::generics::MissingTitle> {
        CreateKeyOptionPostBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _key: core::marker::PhantomData,
            _title: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_current_post_key() -> CreateKeyOptionPostBuilder2<crate::generics::MissingKey, crate::generics::MissingTitle> {
        CreateKeyOptionPostBuilder2 {
            body: Default::default(),
            _key: core::marker::PhantomData,
            _title: core::marker::PhantomData,
        }
    }
}

impl Into<CreateKeyOption> for CreateKeyOptionBuilder<crate::generics::KeyExists, crate::generics::TitleExists> {
    fn into(self) -> CreateKeyOption {
        self.body
    }
}

impl Into<CreateKeyOption> for CreateKeyOptionPostBuilder<crate::generics::UsernameExists, crate::generics::KeyExists, crate::generics::TitleExists> {
    fn into(self) -> CreateKeyOption {
        self.inner.body
    }
}

impl Into<CreateKeyOption> for CreateKeyOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::KeyExists, crate::generics::TitleExists> {
    fn into(self) -> CreateKeyOption {
        self.inner.body
    }
}

impl Into<CreateKeyOption> for CreateKeyOptionPostBuilder2<crate::generics::KeyExists, crate::generics::TitleExists> {
    fn into(self) -> CreateKeyOption {
        self.body
    }
}

/// Builder for [`CreateKeyOption`](./struct.CreateKeyOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateKeyOptionBuilder<Key, Title> {
    body: self::CreateKeyOption,
    _key: core::marker::PhantomData<Key>,
    _title: core::marker::PhantomData<Title>,
}

impl<Key, Title> CreateKeyOptionBuilder<Key, Title> {
    /// An armored SSH key to add
    #[inline]
    pub fn key(mut self, value: impl Into<String>) -> CreateKeyOptionBuilder<crate::generics::KeyExists, Title> {
        self.body.key = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Describe if the key has only read access or read/write
    #[inline]
    pub fn read_only(mut self, value: impl Into<bool>) -> Self {
        self.body.read_only = Some(value.into());
        self
    }

    /// Title of the key to add
    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> CreateKeyOptionBuilder<Key, crate::generics::TitleExists> {
        self.body.title = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`CreateKeyOption::admin_create_public_key`](./struct.CreateKeyOption.html#method.admin_create_public_key) method for a `POST` operation associated with `CreateKeyOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateKeyOptionPostBuilder<Username, Key, Title> {
    inner: CreateKeyOptionPostBuilderContainer,
    _param_username: core::marker::PhantomData<Username>,
    _key: core::marker::PhantomData<Key>,
    _title: core::marker::PhantomData<Title>,
}

#[derive(Debug, Default, Clone)]
struct CreateKeyOptionPostBuilderContainer {
    body: self::CreateKeyOption,
    param_username: Option<String>,
}

impl<Username, Key, Title> CreateKeyOptionPostBuilder<Username, Key, Title> {
    /// username of the user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder<crate::generics::UsernameExists, Key, Title> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// An armored SSH key to add
    #[inline]
    pub fn key(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder<Username, crate::generics::KeyExists, Title> {
        self.inner.body.key = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Describe if the key has only read access or read/write
    #[inline]
    pub fn read_only(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.read_only = Some(value.into());
        self
    }

    /// Title of the key to add
    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder<Username, Key, crate::generics::TitleExists> {
        self.inner.body.title = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateKeyOptionPostBuilder<crate::generics::UsernameExists, crate::generics::KeyExists, crate::generics::TitleExists> {
    type Output = crate::public_key::PublicKey;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/admin/users/{username}/keys", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

/// Builder created by [`CreateKeyOption::repo_create_key`](./struct.CreateKeyOption.html#method.repo_create_key) method for a `POST` operation associated with `CreateKeyOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateKeyOptionPostBuilder1<Owner, Repo, Key, Title> {
    inner: CreateKeyOptionPostBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _key: core::marker::PhantomData<Key>,
    _title: core::marker::PhantomData<Title>,
}

#[derive(Debug, Default, Clone)]
struct CreateKeyOptionPostBuilder1Container {
    body: self::CreateKeyOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo, Key, Title> CreateKeyOptionPostBuilder1<Owner, Repo, Key, Title> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder1<crate::generics::OwnerExists, Repo, Key, Title> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder1<Owner, crate::generics::RepoExists, Key, Title> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// An armored SSH key to add
    #[inline]
    pub fn key(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder1<Owner, Repo, crate::generics::KeyExists, Title> {
        self.inner.body.key = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Describe if the key has only read access or read/write
    #[inline]
    pub fn read_only(mut self, value: impl Into<bool>) -> Self {
        self.inner.body.read_only = Some(value.into());
        self
    }

    /// Title of the key to add
    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder1<Owner, Repo, Key, crate::generics::TitleExists> {
        self.inner.body.title = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateKeyOptionPostBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::KeyExists, crate::generics::TitleExists> {
    type Output = crate::get_repos_owner_repo_keys_id_response::GetReposOwnerRepoKeysIdResponse;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/keys", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}

/// Builder created by [`CreateKeyOption::user_current_post_key`](./struct.CreateKeyOption.html#method.user_current_post_key) method for a `POST` operation associated with `CreateKeyOption`.
#[derive(Debug, Clone)]
pub struct CreateKeyOptionPostBuilder2<Key, Title> {
    body: self::CreateKeyOption,
    _key: core::marker::PhantomData<Key>,
    _title: core::marker::PhantomData<Title>,
}

impl<Key, Title> CreateKeyOptionPostBuilder2<Key, Title> {
    /// An armored SSH key to add
    #[inline]
    pub fn key(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder2<crate::generics::KeyExists, Title> {
        self.body.key = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Describe if the key has only read access or read/write
    #[inline]
    pub fn read_only(mut self, value: impl Into<bool>) -> Self {
        self.body.read_only = Some(value.into());
        self
    }

    /// Title of the key to add
    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> CreateKeyOptionPostBuilder2<Key, crate::generics::TitleExists> {
        self.body.title = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateKeyOptionPostBuilder2<crate::generics::KeyExists, crate::generics::TitleExists> {
    type Output = crate::public_key::PublicKey;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/keys".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body))
    }
}
