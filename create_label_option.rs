
/// CreateLabelOption options for creating a label
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateLabelOption {
    pub color: String,
    pub description: Option<String>,
    pub name: String,
}

impl CreateLabelOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreateLabelOptionBuilder<crate::generics::MissingColor, crate::generics::MissingName> {
        CreateLabelOptionBuilder {
            body: Default::default(),
            _color: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn issue_create_label() -> CreateLabelOptionPostBuilder<crate::generics::MissingOwner, crate::generics::MissingRepo, crate::generics::MissingColor, crate::generics::MissingName> {
        CreateLabelOptionPostBuilder {
            inner: Default::default(),
            _param_owner: core::marker::PhantomData,
            _param_repo: core::marker::PhantomData,
            _color: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }
}

impl Into<CreateLabelOption> for CreateLabelOptionBuilder<crate::generics::ColorExists, crate::generics::NameExists> {
    fn into(self) -> CreateLabelOption {
        self.body
    }
}

impl Into<CreateLabelOption> for CreateLabelOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ColorExists, crate::generics::NameExists> {
    fn into(self) -> CreateLabelOption {
        self.inner.body
    }
}

/// Builder for [`CreateLabelOption`](./struct.CreateLabelOption.html) object.
#[derive(Debug, Clone)]
pub struct CreateLabelOptionBuilder<Color, Name> {
    body: self::CreateLabelOption,
    _color: core::marker::PhantomData<Color>,
    _name: core::marker::PhantomData<Name>,
}

impl<Color, Name> CreateLabelOptionBuilder<Color, Name> {
    #[inline]
    pub fn color(mut self, value: impl Into<String>) -> CreateLabelOptionBuilder<crate::generics::ColorExists, Name> {
        self.body.color = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> CreateLabelOptionBuilder<Color, crate::generics::NameExists> {
        self.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

/// Builder created by [`CreateLabelOption::issue_create_label`](./struct.CreateLabelOption.html#method.issue_create_label) method for a `POST` operation associated with `CreateLabelOption`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CreateLabelOptionPostBuilder<Owner, Repo, Color, Name> {
    inner: CreateLabelOptionPostBuilderContainer,
    _param_owner: core::marker::PhantomData<Owner>,
    _param_repo: core::marker::PhantomData<Repo>,
    _color: core::marker::PhantomData<Color>,
    _name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct CreateLabelOptionPostBuilderContainer {
    body: self::CreateLabelOption,
    param_owner: Option<String>,
    param_repo: Option<String>,
}

impl<Owner, Repo, Color, Name> CreateLabelOptionPostBuilder<Owner, Repo, Color, Name> {
    /// owner of the repo
    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> CreateLabelOptionPostBuilder<crate::generics::OwnerExists, Repo, Color, Name> {
        self.inner.param_owner = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// name of the repo
    #[inline]
    pub fn repo(mut self, value: impl Into<String>) -> CreateLabelOptionPostBuilder<Owner, crate::generics::RepoExists, Color, Name> {
        self.inner.param_repo = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn color(mut self, value: impl Into<String>) -> CreateLabelOptionPostBuilder<Owner, Repo, crate::generics::ColorExists, Name> {
        self.inner.body.color = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.inner.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> CreateLabelOptionPostBuilder<Owner, Repo, Color, crate::generics::NameExists> {
        self.inner.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for CreateLabelOptionPostBuilder<crate::generics::OwnerExists, crate::generics::RepoExists, crate::generics::ColorExists, crate::generics::NameExists> {
    type Output = crate::label::Label;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/repos/{owner}/{repo}/labels", owner=self.inner.param_owner.as_ref().expect("missing parameter owner?"), repo=self.inner.param_repo.as_ref().expect("missing parameter repo?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.inner.body))
    }
}
