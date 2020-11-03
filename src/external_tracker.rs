
/// ExternalTracker represents settings for external tracker
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExternalTracker {
    /// External Issue Tracker URL Format. Use the placeholders {user}, {repo} and {index} for the username, repository name and issue index.
    pub external_tracker_format: Option<String>,
    /// External Issue Tracker Number Format, either `numeric` or `alphanumeric`
    pub external_tracker_style: Option<String>,
    /// URL of external issue tracker.
    pub external_tracker_url: Option<String>,
}

impl ExternalTracker {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ExternalTrackerBuilder {
        ExternalTrackerBuilder {
            body: Default::default(),
        }
    }
}

impl Into<ExternalTracker> for ExternalTrackerBuilder {
    fn into(self) -> ExternalTracker {
        self.body
    }
}

/// Builder for [`ExternalTracker`](./struct.ExternalTracker.html) object.
#[derive(Debug, Clone)]
pub struct ExternalTrackerBuilder {
    body: self::ExternalTracker,
}

impl ExternalTrackerBuilder {
    /// External Issue Tracker URL Format. Use the placeholders {user}, {repo} and {index} for the username, repository name and issue index.
    #[inline]
    pub fn external_tracker_format(mut self, value: impl Into<String>) -> Self {
        self.body.external_tracker_format = Some(value.into());
        self
    }

    /// External Issue Tracker Number Format, either `numeric` or `alphanumeric`
    #[inline]
    pub fn external_tracker_style(mut self, value: impl Into<String>) -> Self {
        self.body.external_tracker_style = Some(value.into());
        self
    }

    /// URL of external issue tracker.
    #[inline]
    pub fn external_tracker_url(mut self, value: impl Into<String>) -> Self {
        self.body.external_tracker_url = Some(value.into());
        self
    }
}
