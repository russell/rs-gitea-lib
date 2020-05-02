
/// DeleteEmailOption options when deleting email addresses
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteEmailOption {
    /// email addresses to delete
    pub emails: Option<Vec<String>>,
}

impl DeleteEmailOption {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> DeleteEmailOptionBuilder {
        DeleteEmailOptionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn user_delete_email() -> DeleteEmailOptionDeleteBuilder {
        DeleteEmailOptionDeleteBuilder {
            body: Default::default(),
        }
    }
}

impl Into<DeleteEmailOption> for DeleteEmailOptionBuilder {
    fn into(self) -> DeleteEmailOption {
        self.body
    }
}

impl Into<DeleteEmailOption> for DeleteEmailOptionDeleteBuilder {
    fn into(self) -> DeleteEmailOption {
        self.body
    }
}

/// Builder for [`DeleteEmailOption`](./struct.DeleteEmailOption.html) object.
#[derive(Debug, Clone)]
pub struct DeleteEmailOptionBuilder {
    body: self::DeleteEmailOption,
}

impl DeleteEmailOptionBuilder {
    /// email addresses to delete
    #[inline]
    pub fn emails(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.emails = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

/// Builder created by [`DeleteEmailOption::user_delete_email`](./struct.DeleteEmailOption.html#method.user_delete_email) method for a `DELETE` operation associated with `DeleteEmailOption`.
#[derive(Debug, Clone)]
pub struct DeleteEmailOptionDeleteBuilder {
    body: self::DeleteEmailOption,
}

impl DeleteEmailOptionDeleteBuilder {
    /// email addresses to delete
    #[inline]
    pub fn emails(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.emails = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for DeleteEmailOptionDeleteBuilder {
    type Output = serde_json::Value;

    const METHOD: http::Method = http::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/user/emails".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .json(&self.body)
        .header(http::header::ACCEPT.as_str(), "application/json"))
    }
}
