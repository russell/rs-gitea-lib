
/// IssueDeadline represents an issue deadline
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssueDeadline {
    pub due_date: Option<String>,
}

impl IssueDeadline {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> IssueDeadlineBuilder {
        IssueDeadlineBuilder {
            body: Default::default(),
        }
    }
}

impl Into<IssueDeadline> for IssueDeadlineBuilder {
    fn into(self) -> IssueDeadline {
        self.body
    }
}

/// Builder for [`IssueDeadline`](./struct.IssueDeadline.html) object.
#[derive(Debug, Clone)]
pub struct IssueDeadlineBuilder {
    body: self::IssueDeadline,
}

impl IssueDeadlineBuilder {
    #[inline]
    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.body.due_date = Some(value.into());
        self
    }
}
