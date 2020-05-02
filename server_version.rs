
/// ServerVersion wraps the version of the server
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ServerVersion {
    pub version: Option<String>,
}

impl ServerVersion {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ServerVersionBuilder {
        ServerVersionBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get_version() -> ServerVersionGetBuilder {
        ServerVersionGetBuilder
    }
}

impl Into<ServerVersion> for ServerVersionBuilder {
    fn into(self) -> ServerVersion {
        self.body
    }
}

/// Builder for [`ServerVersion`](./struct.ServerVersion.html) object.
#[derive(Debug, Clone)]
pub struct ServerVersionBuilder {
    body: self::ServerVersion,
}

impl ServerVersionBuilder {
    #[inline]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.body.version = Some(value.into());
        self
    }
}

/// Builder created by [`ServerVersion::get_version`](./struct.ServerVersion.html#method.get_version) method for a `GET` operation associated with `ServerVersion`.
#[derive(Debug, Clone)]
pub struct ServerVersionGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for ServerVersionGetBuilder {
    type Output = ServerVersion;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/version".into()
    }
}
