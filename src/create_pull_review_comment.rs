
/// CreatePullReviewComment represent a review comment for creation api
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreatePullReviewComment {
    pub body: Option<String>,
    /// if comment to new file line or 0
    pub new_position: Option<i64>,
    /// if comment to old file line or 0
    pub old_position: Option<i64>,
    /// the tree path
    pub path: Option<String>,
}

impl CreatePullReviewComment {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CreatePullReviewCommentBuilder {
        CreatePullReviewCommentBuilder {
            body: Default::default(),
        }
    }
}

impl Into<CreatePullReviewComment> for CreatePullReviewCommentBuilder {
    fn into(self) -> CreatePullReviewComment {
        self.body
    }
}

/// Builder for [`CreatePullReviewComment`](./struct.CreatePullReviewComment.html) object.
#[derive(Debug, Clone)]
pub struct CreatePullReviewCommentBuilder {
    body: self::CreatePullReviewComment,
}

impl CreatePullReviewCommentBuilder {
    #[inline]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body.body = Some(value.into());
        self
    }

    /// if comment to new file line or 0
    #[inline]
    pub fn new_position(mut self, value: impl Into<i64>) -> Self {
        self.body.new_position = Some(value.into());
        self
    }

    /// if comment to old file line or 0
    #[inline]
    pub fn old_position(mut self, value: impl Into<i64>) -> Self {
        self.body.old_position = Some(value.into());
        self
    }

    /// the tree path
    #[inline]
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.body.path = Some(value.into());
        self
    }
}
