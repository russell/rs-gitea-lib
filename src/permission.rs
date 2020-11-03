
/// Permission represents a set of permissions
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub admin: Option<bool>,
    pub pull: Option<bool>,
    pub push: Option<bool>,
}

impl Permission {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PermissionBuilder {
        PermissionBuilder {
            body: Default::default(),
        }
    }
}

impl Into<Permission> for PermissionBuilder {
    fn into(self) -> Permission {
        self.body
    }
}

/// Builder for [`Permission`](./struct.Permission.html) object.
#[derive(Debug, Clone)]
pub struct PermissionBuilder {
    body: self::Permission,
}

impl PermissionBuilder {
    #[inline]
    pub fn admin(mut self, value: impl Into<bool>) -> Self {
        self.body.admin = Some(value.into());
        self
    }

    #[inline]
    pub fn pull(mut self, value: impl Into<bool>) -> Self {
        self.body.pull = Some(value.into());
        self
    }

    #[inline]
    pub fn push(mut self, value: impl Into<bool>) -> Self {
        self.body.push = Some(value.into());
        self
    }
}
