
/// User represents a user
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    /// URL to the user's avatar
    pub avatar_url: Option<String>,
    pub created: Option<String>,
    pub email: Option<String>,
    /// the user's full name
    pub full_name: Option<String>,
    /// the user's id
    pub id: Option<i64>,
    /// Is the user an administrator
    pub is_admin: Option<bool>,
    /// User locale
    pub language: Option<String>,
    pub last_login: Option<String>,
    /// the user's username
    pub login: Option<String>,
}

impl User {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> UserBuilder {
        UserBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn admin_get_all_users() -> UserGetBuilder {
        UserGetBuilder
    }

    #[inline]
    pub fn org_list_members() -> UserGetBuilder1<crate::generics::MissingOrg> {
        UserGetBuilder1 {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_list_public_members() -> UserGetBuilder2<crate::generics::MissingOrg> {
        UserGetBuilder2 {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_list_collaborators() -> UserGetBuilder3<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        UserGetBuilder3 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_subscriptions() -> UserGetBuilder4<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingIndex> {
        UserGetBuilder4 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _param_index: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_list_stargazers() -> UserGetBuilder5<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        UserGetBuilder5 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_list_subscribers() -> UserGetBuilder6<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        UserGetBuilder6 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_list_team_members() -> UserGetBuilder7<crate::generics::MissingId> {
        UserGetBuilder7 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_list_team_member() -> UserGetBuilder8<crate::generics::MissingId, crate::generics::MissingUsername> {
        UserGetBuilder8 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
            _param_username: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_get_current() -> UserGetBuilder9 {
        UserGetBuilder9
    }

    #[inline]
    pub fn user_current_list_followers() -> UserGetBuilder10 {
        UserGetBuilder10
    }

    #[inline]
    pub fn user_current_list_following() -> UserGetBuilder11 {
        UserGetBuilder11
    }

    #[inline]
    pub fn user_get() -> UserGetBuilder12<crate::generics::MissingUsername> {
        UserGetBuilder12 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_followers() -> UserGetBuilder13<crate::generics::MissingUsername> {
        UserGetBuilder13 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_following() -> UserGetBuilder14<crate::generics::MissingUsername> {
        UserGetBuilder14 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }
}

impl Into<User> for UserBuilder {
    fn into(self) -> User {
        self.body
    }
}

/// Builder for [`User`](./struct.User.html) object.
#[derive(Debug, Clone)]
pub struct UserBuilder {
    body: self::User,
}

impl UserBuilder {
    /// URL to the user's avatar
    #[inline]
    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.body.avatar_url = Some(value.into());
        self
    }

    #[inline]
    pub fn created(mut self, value: impl Into<String>) -> Self {
        self.body.created = Some(value.into());
        self
    }

    #[inline]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.body.email = Some(value.into());
        self
    }

    /// the user's full name
    #[inline]
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.body.full_name = Some(value.into());
        self
    }

    /// the user's id
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    /// Is the user an administrator
    #[inline]
    pub fn is_admin(mut self, value: impl Into<bool>) -> Self {
        self.body.is_admin = Some(value.into());
        self
    }

    /// User locale
    #[inline]
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.body.language = Some(value.into());
        self
    }

    #[inline]
    pub fn last_login(mut self, value: impl Into<String>) -> Self {
        self.body.last_login = Some(value.into());
        self
    }

    /// the user's username
    #[inline]
    pub fn login(mut self, value: impl Into<String>) -> Self {
        self.body.login = Some(value.into());
        self
    }
}

/// Builder created by [`User::admin_get_all_users`](./struct.User.html#method.admin_get_all_users) method for a `GET` operation associated with `User`.
#[derive(Debug, Clone)]
pub struct UserGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/admin/users".into()
    }
}

impl crate::client::ResponseWrapper<Vec<User>, UserGetBuilder> {
    #[inline]
    pub fn message(&self) -> Option<String> {
        self.headers.get("message").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn url(&self) -> Option<String> {
        self.headers.get("url").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`User::org_list_members`](./struct.User.html#method.org_list_members) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder1<Org> {
    inner: UserGetBuilder1Container,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder1Container {
    param_org: Option<String>,
}

impl<Org> UserGetBuilder1<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> UserGetBuilder1<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder1<crate::generics::OrgExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/members", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }
}

/// Builder created by [`User::org_list_public_members`](./struct.User.html#method.org_list_public_members) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder2<Org> {
    inner: UserGetBuilder2Container,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder2Container {
    param_org: Option<String>,
}

impl<Org> UserGetBuilder2<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> UserGetBuilder2<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder2<crate::generics::OrgExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/public_members", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
    }
}

/// Builder created by [`User::repo_list_collaborators`](./struct.User.html#method.repo_list_collaborators) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder3<Owner, Repo> {
    inner: UserGetBuilder3Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder3Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> UserGetBuilder3<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> UserGetBuilder3<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> UserGetBuilder3<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder3<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/collaborators", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`User::issue_subscriptions`](./struct.User.html#method.issue_subscriptions) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder4<Owner, Repo, Index> {
    inner: UserGetBuilder4Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _param_index: core::marker::PhantomData<Index>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder4Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_index: Option<i64>,
}

impl<Owner, Repo, Index> UserGetBuilder4<Owner, Repo, Index> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> UserGetBuilder4<crate::generics::OwnerExists, Repo, Index> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> UserGetBuilder4<Owner, crate::generics::RepoExists, Index> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// index of the issue
    #[inline]
    pub fn index(mut self, value: impl Into<i64>) -> UserGetBuilder4<Owner, Repo, crate::generics::IndexExists> {
        self.inner.param_index = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder4<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::IndexExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issues/{index}/subscriptions", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?"), index=self.inner.param_index.as_ref().expect("missing parameter index?")).into()
    }
}

/// Builder created by [`User::repo_list_stargazers`](./struct.User.html#method.repo_list_stargazers) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder5<Owner, Repo> {
    inner: UserGetBuilder5Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder5Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> UserGetBuilder5<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> UserGetBuilder5<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> UserGetBuilder5<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder5<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/stargazers", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`User::repo_list_subscribers`](./struct.User.html#method.repo_list_subscribers) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder6<Owner, Repo> {
    inner: UserGetBuilder6Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder6Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> UserGetBuilder6<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> UserGetBuilder6<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> UserGetBuilder6<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder6<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/subscribers", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`User::org_list_team_members`](./struct.User.html#method.org_list_team_members) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder7<Id> {
    inner: UserGetBuilder7Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder7Container {
    param_id: Option<i64>,
}

impl<Id> UserGetBuilder7<Id> {
    /// id of the team
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> UserGetBuilder7<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder7<crate::generics::IdExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/teams/{id}/members", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`User::org_list_team_member`](./struct.User.html#method.org_list_team_member) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder8<Id, Username> {
    inner: UserGetBuilder8Container,
    _param_id: core::marker::PhantomData<Id>,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder8Container {
    param_id: Option<i64>,
    param_username: Option<String>,
}

impl<Id, Username> UserGetBuilder8<Id, Username> {
    /// id of the team
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> UserGetBuilder8<crate::generics::IdExists, Username> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// username of the member to list
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> UserGetBuilder8<Id, crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder8<crate::generics::IdExists, crate::generics::UsernameExists> {
    type Output = User;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/teams/{id}/members/{username}", id=self.inner.param_id.as_ref().expect("missing parameter id?"), username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }
}

/// Builder created by [`User::user_get_current`](./struct.User.html#method.user_get_current) method for a `GET` operation associated with `User`.
#[derive(Debug, Clone)]
pub struct UserGetBuilder9;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder9 {
    type Output = User;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user".into()
    }
}

/// Builder created by [`User::user_current_list_followers`](./struct.User.html#method.user_current_list_followers) method for a `GET` operation associated with `User`.
#[derive(Debug, Clone)]
pub struct UserGetBuilder10;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder10 {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/followers".into()
    }
}

/// Builder created by [`User::user_current_list_following`](./struct.User.html#method.user_current_list_following) method for a `GET` operation associated with `User`.
#[derive(Debug, Clone)]
pub struct UserGetBuilder11;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder11 {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/following".into()
    }
}

/// Builder created by [`User::user_get`](./struct.User.html#method.user_get) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder12<Username> {
    inner: UserGetBuilder12Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder12Container {
    param_username: Option<String>,
}

impl<Username> UserGetBuilder12<Username> {
    /// username of user to get
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> UserGetBuilder12<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder12<crate::generics::UsernameExists> {
    type Output = User;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }
}

/// Builder created by [`User::user_list_followers`](./struct.User.html#method.user_list_followers) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder13<Username> {
    inner: UserGetBuilder13Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder13Container {
    param_username: Option<String>,
}

impl<Username> UserGetBuilder13<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> UserGetBuilder13<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder13<crate::generics::UsernameExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/followers", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }
}

/// Builder created by [`User::user_list_following`](./struct.User.html#method.user_list_following) method for a `GET` operation associated with `User`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserGetBuilder14<Username> {
    inner: UserGetBuilder14Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct UserGetBuilder14Container {
    param_username: Option<String>,
}

impl<Username> UserGetBuilder14<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> UserGetBuilder14<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for UserGetBuilder14<crate::generics::UsernameExists> {
    type Output = Vec<User>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/following", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
    }
}
