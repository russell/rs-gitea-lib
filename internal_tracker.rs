
/// InternalTracker represents settings for internal tracker
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InternalTracker {
    /// Let only contributors track time (Built-in issue tracker)
    pub allow_only_contributors_to_track_time: Option<bool>,
    /// Enable dependencies for issues and pull requests (Built-in issue tracker)
    pub enable_issue_dependencies: Option<bool>,
    /// Enable time tracking (Built-in issue tracker)
    pub enable_time_tracker: Option<bool>,
}

impl InternalTracker {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> InternalTrackerBuilder {
        InternalTrackerBuilder {
            body: Default::default(),
        }
    }
}

impl Into<InternalTracker> for InternalTrackerBuilder {
    fn into(self) -> InternalTracker {
        self.body
    }
}

/// Builder for [`InternalTracker`](./struct.InternalTracker.html) object.
#[derive(Debug, Clone)]
pub struct InternalTrackerBuilder {
    body: self::InternalTracker,
}

impl InternalTrackerBuilder {
    /// Let only contributors track time (Built-in issue tracker)
    #[inline]
    pub fn allow_only_contributors_to_track_time(mut self, value: impl Into<bool>) -> Self {
        self.body.allow_only_contributors_to_track_time = Some(value.into());
        self
    }

    /// Enable dependencies for issues and pull requests (Built-in issue tracker)
    #[inline]
    pub fn enable_issue_dependencies(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_issue_dependencies = Some(value.into());
        self
    }

    /// Enable time tracking (Built-in issue tracker)
    #[inline]
    pub fn enable_time_tracker(mut self, value: impl Into<bool>) -> Self {
        self.body.enable_time_tracker = Some(value.into());
        self
    }
}
