
/// NotificationSubject contains the notification subject (Issue/Pull/Commit)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotificationSubject {
    pub latest_comment_url: Option<String>,
    pub state: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
}

impl NotificationSubject {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> NotificationSubjectBuilder {
        NotificationSubjectBuilder {
            body: Default::default(),
        }
    }
}

impl Into<NotificationSubject> for NotificationSubjectBuilder {
    fn into(self) -> NotificationSubject {
        self.body
    }
}

/// Builder for [`NotificationSubject`](./struct.NotificationSubject.html) object.
#[derive(Debug, Clone)]
pub struct NotificationSubjectBuilder {
    body: self::NotificationSubject,
}

impl NotificationSubjectBuilder {
    #[inline]
    pub fn latest_comment_url(mut self, value: impl Into<String>) -> Self {
        self.body.latest_comment_url = Some(value.into());
        self
    }

    #[inline]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.body.state = Some(value.into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.body.title = Some(value.into());
        self
    }

    #[inline]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.body.type_ = Some(value.into());
        self
    }

    #[inline]
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.body.url = Some(value.into());
        self
    }
}
