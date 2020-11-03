
/// NotificationCount number of unread notifications
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotificationCount {
    pub new: Option<i64>,
}

impl NotificationCount {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> NotificationCountBuilder {
        NotificationCountBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn notify_new_available() -> NotificationCountGetBuilder {
        NotificationCountGetBuilder
    }
}

impl Into<NotificationCount> for NotificationCountBuilder {
    fn into(self) -> NotificationCount {
        self.body
    }
}

/// Builder for [`NotificationCount`](./struct.NotificationCount.html) object.
#[derive(Debug, Clone)]
pub struct NotificationCountBuilder {
    body: self::NotificationCount,
}

impl NotificationCountBuilder {
    #[inline]
    pub fn new(mut self, value: impl Into<i64>) -> Self {
        self.body.new = Some(value.into());
        self
    }
}

/// Builder created by [`NotificationCount::notify_new_available`](./struct.NotificationCount.html#method.notify_new_available) method for a `GET` operation associated with `NotificationCount`.
#[derive(Debug, Clone)]
pub struct NotificationCountGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for NotificationCountGetBuilder {
    type Output = NotificationCount;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/notifications/new".into()
    }
}
