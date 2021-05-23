
/// CommitAffectedFiles store information about files affected by the commit
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommitAffectedFiles {
    pub filename: Option<String>,
}

impl CommitAffectedFiles {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CommitAffectedFilesBuilder {
        CommitAffectedFilesBuilder {
            body: Default::default(),
        }
    }
}

impl Into<CommitAffectedFiles> for CommitAffectedFilesBuilder {
    fn into(self) -> CommitAffectedFiles {
        self.body
    }
}

/// Builder for [`CommitAffectedFiles`](./struct.CommitAffectedFiles.html) object.
#[derive(Debug, Clone)]
pub struct CommitAffectedFilesBuilder {
    body: self::CommitAffectedFiles,
}

impl CommitAffectedFilesBuilder {
    #[inline]
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.body.filename = Some(value.into());
        self
    }
}
