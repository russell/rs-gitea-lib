
/// RepositoryMeta basic repository information
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RepositoryMeta {
    pub full_name: Option<String>,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub owner: Option<String>,
}

impl RepositoryMeta {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> RepositoryMetaBuilder {
        RepositoryMetaBuilder {
            body: Default::default(),
        }
    }
}

impl Into<RepositoryMeta> for RepositoryMetaBuilder {
    fn into(self) -> RepositoryMeta {
        self.body
    }
}

/// Builder for [`RepositoryMeta`](./struct.RepositoryMeta.html) object.
#[derive(Debug, Clone)]
pub struct RepositoryMetaBuilder {
    body: self::RepositoryMeta,
}

impl RepositoryMetaBuilder {
    #[inline]
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.body.full_name = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }

    #[inline]
    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.body.owner = Some(value.into());
        self
    }
}
