
/// Repository represents a repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub allow_merge_commits: Option<bool>,
    pub allow_rebase: Option<bool>,
    pub allow_rebase_explicit: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub archived: Option<bool>,
    pub avatar_url: Option<String>,
    pub clone_url: Option<String>,
    pub created_at: Option<String>,
    pub default_branch: Option<String>,
    pub description: Option<String>,
    pub empty: Option<bool>,
    pub external_tracker: Option<crate::external_tracker::ExternalTracker>,
    pub external_wiki: Option<crate::external_wiki::ExternalWiki>,
    pub fork: Option<bool>,
    pub forks_count: Option<i64>,
    pub full_name: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_pull_requests: Option<bool>,
    pub has_wiki: Option<bool>,
    pub html_url: Option<String>,
    pub id: Option<i64>,
    pub ignore_whitespace_conflicts: Option<bool>,
    pub internal: Option<bool>,
    pub internal_tracker: Option<crate::internal_tracker::InternalTracker>,
    pub mirror: Option<bool>,
    pub name: Option<String>,
    pub open_issues_count: Option<i64>,
    pub open_pr_counter: Option<i64>,
    pub original_url: Option<String>,
    pub owner: Option<crate::user::User>,
    pub parent: Option<Box<crate::repository::Repository>>,
    pub permissions: Option<crate::permission::Permission>,
    pub private: Option<bool>,
    pub release_counter: Option<i64>,
    pub size: Option<i64>,
    pub ssh_url: Option<String>,
    pub stars_count: Option<i64>,
    pub template: Option<bool>,
    pub updated_at: Option<String>,
    pub watchers_count: Option<i64>,
    pub website: Option<String>,
}

impl Repository {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> RepositoryBuilder {
        RepositoryBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn org_list_repos() -> RepositoryGetBuilder<crate::generics::MissingOrg> {
        RepositoryGetBuilder {
            inner: Default::default(),
            _param_org: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get() -> RepositoryGetBuilder1<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        RepositoryGetBuilder1 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn list_forks() -> RepositoryGetBuilder2<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        RepositoryGetBuilder2 {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn repo_get_by_id() -> RepositoryGetBuilder3<crate::generics::MissingId> {
        RepositoryGetBuilder3 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn org_list_team_repos() -> RepositoryGetBuilder4<crate::generics::MissingId> {
        RepositoryGetBuilder4 {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_current_list_repos() -> RepositoryGetBuilder5 {
        RepositoryGetBuilder5 {
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn user_current_list_starred() -> RepositoryGetBuilder6 {
        RepositoryGetBuilder6 {
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn user_current_list_subscriptions() -> RepositoryGetBuilder7 {
        RepositoryGetBuilder7 {
            param_page: None,
            param_limit: None,
        }
    }

    #[inline]
    pub fn user_list_repos() -> RepositoryGetBuilder8<crate::generics::MissingUsername> {
        RepositoryGetBuilder8 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_starred() -> RepositoryGetBuilder9<crate::generics::MissingUsername> {
        RepositoryGetBuilder9 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn user_list_subscriptions() -> RepositoryGetBuilder10<crate::generics::MissingUsername> {
        RepositoryGetBuilder10 {
            inner: Default::default(),
            _param_username: core::marker::PhantomData,
        }
    }
}

impl Into<Repository> for RepositoryBuilder {
    fn into(self) -> Repository {
        self.body
    }
}

/// Builder for [`Repository`](./struct.Repository.html) object.
#[derive(Debug, Clone)]
pub struct RepositoryBuilder {
    body: self::Repository,
}

impl RepositoryBuilder {
    #[inline]
    pub fn allow_merge_commits(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_merge_commits = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_rebase(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_rebase = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_rebase_explicit(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_rebase_explicit = Some(value.into());
        self
    }

    #[inline]
    pub fn allow_squash_merge(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_squash_merge = Some(value.into());
        self
    }

    #[inline]
    pub fn archived(mut self, value: impl Into<bool>) -> Self {
        self.body.archived = Some(value.into());
        self
    }

    #[inline]
    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.body.avatar_url = Some(value.into());
        self
    }

    #[inline]
    pub fn clone_url(mut self, value: impl Into<String>) -> Self {
        self.body.clone_url = Some(value.into());
        self
    }

    #[inline]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.body.created_at = Some(value.into());
        self
    }

    #[inline]
    pub fn default_branch(mut self, value: impl Into<String>) -> Self {
        self.body.default_branch = Some(value.into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn empty(mut self, value: impl Into<bool>) -> Self {
        self.body.empty = Some(value.into());
        self
    }

    #[inline]
    pub fn external_tracker(mut self, value: crate::external_tracker::ExternalTracker) -> Self {
        self.body.external_tracker = Some(value.into());
        self
    }

    #[inline]
    pub fn external_wiki(mut self, value: crate::external_wiki::ExternalWiki) -> Self {
        self.body.external_wiki = Some(value.into());
        self
    }

    #[inline]
    pub fn fork(mut self, value: impl Into<bool>) -> Self {
        self.body.fork = Some(value.into());
        self
    }

    #[inline]
    pub fn forks_count(mut self, value: impl Into<i64>) -> Self {
        self.body.forks_count = Some(value.into());
        self
    }

    #[inline]
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.body.full_name = Some(value.into());
        self
    }

    #[inline]
    pub fn has_issues(mut self, value: impl Into<bool>) -> Self {
        self.body.has_issues = Some(value.into());
        self
    }

    #[inline]
    pub fn has_projects(mut self, value: impl Into<bool>) -> Self {
        self.body.has_projects = Some(value.into());
        self
    }

    #[inline]
    pub fn has_pull_requests(mut self, value: impl Into<bool>) -> Self {
        self.body.has_pull_requests = Some(value.into());
        self
    }

    #[inline]
    pub fn has_wiki(mut self, value: impl Into<bool>) -> Self {
        self.body.has_wiki = Some(value.into());
        self
    }

    #[inline]
    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.body.html_url = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn ignore_whitespace_conflicts(mut self, value: impl Into<bool>) -> Self {
        self.body.ignore_whitespace_conflicts = Some(value.into());
        self
    }

    #[inline]
    pub fn internal(mut self, value: impl Into<bool>) -> Self {
        self.body.internal = Some(value.into());
        self
    }

    #[inline]
    pub fn internal_tracker(mut self, value: crate::internal_tracker::InternalTracker) -> Self {
        self.body.internal_tracker = Some(value.into());
        self
    }

    #[inline]
    pub fn mirror(mut self, value: impl Into<bool>) -> Self {
        self.body.mirror = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn open_issues_count(mut self, value: impl Into<i64>) -> Self {
        self.body.open_issues_count = Some(value.into());
        self
    }

    #[inline]
    pub fn open_pr_counter(mut self, value: impl Into<i64>) -> Self {
        self.body.open_pr_counter = Some(value.into());
        self
    }

    #[inline]
    pub fn original_url(mut self, value: impl Into<String>) -> Self {
        self.body.original_url = Some(value.into());
        self
    }

    #[inline]
    pub fn owner(mut self, value: crate::user::User) -> Self {
        self.body.owner = Some(value.into());
        self
    }

    #[inline]
    pub fn parent(mut self, value: crate::repository::Repository) -> Self {
        self.body.parent = Some(value.into());
        self
    }

    #[inline]
    pub fn permissions(mut self, value: crate::permission::Permission) -> Self {
        self.body.permissions = Some(value.into());
        self
    }

    #[inline]
    pub fn private(mut self, value: impl Into<bool>) -> Self {
        self.body.private = Some(value.into());
        self
    }

    #[inline]
    pub fn release_counter(mut self, value: impl Into<i64>) -> Self {
        self.body.release_counter = Some(value.into());
        self
    }

    #[inline]
    pub fn size(mut self, value: impl Into<i64>) -> Self {
        self.body.size = Some(value.into());
        self
    }

    #[inline]
    pub fn ssh_url(mut self, value: impl Into<String>) -> Self {
        self.body.ssh_url = Some(value.into());
        self
    }

    #[inline]
    pub fn stars_count(mut self, value: impl Into<i64>) -> Self {
        self.body.stars_count = Some(value.into());
        self
    }

    #[inline]
    pub fn template(mut self, value: impl Into<bool>) -> Self {
        self.body.template = Some(value.into());
        self
    }

    #[inline]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.body.updated_at = Some(value.into());
        self
    }

    #[inline]
    pub fn watchers_count(mut self, value: impl Into<i64>) -> Self {
        self.body.watchers_count = Some(value.into());
        self
    }

    #[inline]
    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.body.website = Some(value.into());
        self
    }
}

/// Builder created by [`Repository::org_list_repos`](./struct.Repository.html#method.org_list_repos) method for a `GET` operation associated with `Repository`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder<Org> {
    inner: RepositoryGetBuilderContainer,
    _param_org: core::marker::PhantomData<Org>,
}

#[derive(Debug, Default, Clone)]
struct RepositoryGetBuilderContainer {
    param_org: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Org> RepositoryGetBuilder<Org> {
    /// name of the organization
    #[inline]
    pub fn org(mut self, value: impl Into<String>) -> RepositoryGetBuilder<crate::generics::OrgExists> {
        self.inner.param_org = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder<crate::generics::OrgExists> {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/orgs/{org}/repos", org=self.inner.param_org.as_ref().expect("missing parameter org?")).into()
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

/// Builder created by [`Repository::repo_get`](./struct.Repository.html#method.repo_get) method for a `GET` operation associated with `Repository`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder1<Owner, Repo> {
    inner: RepositoryGetBuilder1Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct RepositoryGetBuilder1Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> RepositoryGetBuilder1<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> RepositoryGetBuilder1<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> RepositoryGetBuilder1<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder1<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Repository;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}

/// Builder created by [`Repository::list_forks`](./struct.Repository.html#method.list_forks) method for a `GET` operation associated with `Repository`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder2<Owner, Repo> {
    inner: RepositoryGetBuilder2Container,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct RepositoryGetBuilder2Container {
    param_owner: Option<String>,
    param_repo: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Owner, Repo> RepositoryGetBuilder2<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> RepositoryGetBuilder2<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> RepositoryGetBuilder2<Owner, crate::generics::RepoExists> {
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder2<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/forks", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
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

/// Builder created by [`Repository::repo_get_by_id`](./struct.Repository.html#method.repo_get_by_id) method for a `GET` operation associated with `Repository`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder3<Id> {
    inner: RepositoryGetBuilder3Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct RepositoryGetBuilder3Container {
    param_id: Option<i64>,
}

impl<Id> RepositoryGetBuilder3<Id> {
    /// id of the repo to get
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> RepositoryGetBuilder3<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder3<crate::generics::IdExists> {
    type Output = Repository;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repositories/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

/// Builder created by [`Repository::org_list_team_repos`](./struct.Repository.html#method.org_list_team_repos) method for a `GET` operation associated with `Repository`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder4<Id> {
    inner: RepositoryGetBuilder4Container,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct RepositoryGetBuilder4Container {
    param_id: Option<i64>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Id> RepositoryGetBuilder4<Id> {
    /// id of the team
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> RepositoryGetBuilder4<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder4<crate::generics::IdExists> {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/teams/{id}/repos", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
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

/// Builder created by [`Repository::user_current_list_repos`](./struct.Repository.html#method.user_current_list_repos) method for a `GET` operation associated with `Repository`.
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder5 {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl RepositoryGetBuilder5 {
    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder5 {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/repos".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Repository::user_current_list_starred`](./struct.Repository.html#method.user_current_list_starred) method for a `GET` operation associated with `Repository`.
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder6 {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl RepositoryGetBuilder6 {
    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder6 {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/starred".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Repository::user_current_list_subscriptions`](./struct.Repository.html#method.user_current_list_subscriptions) method for a `GET` operation associated with `Repository`.
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder7 {
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl RepositoryGetBuilder7 {
    /// page number of results to return (1-based)
    #[inline]
    pub fn page(mut self, value: impl Into<i64>) -> Self {
        self.param_page = Some(value.into());
        self
    }

    /// page size of results
    #[inline]
    pub fn limit(mut self, value: impl Into<i64>) -> Self {
        self.param_limit = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder7 {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/subscriptions".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .query(&[
            ("page", self.param_page.as_ref().map(std::string::ToString::to_string)),
            ("limit", self.param_limit.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}

/// Builder created by [`Repository::user_list_repos`](./struct.Repository.html#method.user_list_repos) method for a `GET` operation associated with `Repository`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder8<Username> {
    inner: RepositoryGetBuilder8Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct RepositoryGetBuilder8Container {
    param_username: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Username> RepositoryGetBuilder8<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> RepositoryGetBuilder8<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder8<crate::generics::UsernameExists> {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/repos", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
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

/// Builder created by [`Repository::user_list_starred`](./struct.Repository.html#method.user_list_starred) method for a `GET` operation associated with `Repository`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder9<Username> {
    inner: RepositoryGetBuilder9Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct RepositoryGetBuilder9Container {
    param_username: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Username> RepositoryGetBuilder9<Username> {
    /// username of user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> RepositoryGetBuilder9<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder9<crate::generics::UsernameExists> {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/starred", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
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

/// Builder created by [`Repository::user_list_subscriptions`](./struct.Repository.html#method.user_list_subscriptions) method for a `GET` operation associated with `Repository`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RepositoryGetBuilder10<Username> {
    inner: RepositoryGetBuilder10Container,
    _param_username: core::marker::PhantomData<Username>,
}

#[derive(Debug, Default, Clone)]
struct RepositoryGetBuilder10Container {
    param_username: Option<String>,
    param_page: Option<i64>,
    param_limit: Option<i64>,
}

impl<Username> RepositoryGetBuilder10<Username> {
    /// username of the user
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> RepositoryGetBuilder10<crate::generics::UsernameExists> {
        self.inner.param_username = Some(value.into());
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RepositoryGetBuilder10<crate::generics::UsernameExists> {
    type Output = Vec<Repository>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/users/{username}/subscriptions", username=self.inner.param_username.as_ref().expect("missing parameter username?")).into()
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
