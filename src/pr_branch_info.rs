
/// PRBranchInfo information about a branch
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PrBranchInfo {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub repo: Option<Box<crate::repository::Repository>>,
    pub repo_id: Option<i64>,
    pub sha: Option<String>,
}

impl PrBranchInfo {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PrBranchInfoBuilder {
        PrBranchInfoBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PrBranchInfo> for PrBranchInfoBuilder {
    fn into(self) -> PrBranchInfo {
        self.body
    }
}

/// Builder for [`PrBranchInfo`](./struct.PrBranchInfo.html) object.
#[derive(Debug, Clone)]
pub struct PrBranchInfoBuilder {
    body: self::PrBranchInfo,
}

impl PrBranchInfoBuilder {
    #[inline]
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.body.label = Some(value.into());
        self
    }

    #[inline]
    pub fn ref_(mut self, value: impl Into<String>) -> Self {
        self.body.ref_ = Some(value.into());
        self
    }

    #[inline]
    pub fn repo(mut self, value: crate::repository::Repository) -> Self {
        self.body.repo = Some(value.into());
        self
    }

    #[inline]
    pub fn repo_id(mut self, value: impl Into<i64>) -> Self {
        self.body.repo_id = Some(value.into());
        self
    }

    #[inline]
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.body.sha = Some(value.into());
        self
    }
}
