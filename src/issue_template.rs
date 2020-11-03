
/// IssueTemplate represents an issue template for a repository
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssueTemplate {
    pub about: Option<String>,
    pub content: Option<String>,
    pub file_name: Option<String>,
    pub labels: Option<Vec<String>>,
    pub name: Option<String>,
    pub title: Option<String>,
}

impl IssueTemplate {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> IssueTemplateBuilder {
        IssueTemplateBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn repo_get_issue_templates() -> IssueTemplateGetBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo> {
        IssueTemplateGetBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
        }
    }
}

impl Into<IssueTemplate> for IssueTemplateBuilder {
    fn into(self) -> IssueTemplate {
        self.body
    }
}

/// Builder for [`IssueTemplate`](./struct.IssueTemplate.html) object.
#[derive(Debug, Clone)]
pub struct IssueTemplateBuilder {
    body: self::IssueTemplate,
}

impl IssueTemplateBuilder {
    #[inline]
    pub fn about(mut self, value: impl Into<String>) -> Self {
        self.body.about = Some(value.into());
        self
    }

    #[inline]
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.body.content = Some(value.into());
        self
    }

    #[inline]
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.body.file_name = Some(value.into());
        self
    }

    #[inline]
    pub fn labels(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.labels = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.body.title = Some(value.into());
        self
    }
}

/// Builder created by [`IssueTemplate::repo_get_issue_templates`](./struct.IssueTemplate.html#method.repo_get_issue_templates) method for a `GET` operation associated with `IssueTemplate`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct IssueTemplateGetBuilder<Owner, Repo> {
    inner: IssueTemplateGetBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
}

#[derive(Debug, Default, Clone)]
struct IssueTemplateGetBuilderContainer {
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo> IssueTemplateGetBuilder<Owner, Repo> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> IssueTemplateGetBuilder<crate::generics::OwnerExists, Repo> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> IssueTemplateGetBuilder<Owner, crate::generics::RepoExists> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for IssueTemplateGetBuilder<crate::generics::OwnerExists, crate::generics::RepoExists> {
    type Output = Vec<IssueTemplate>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/issue_templates", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }
}
