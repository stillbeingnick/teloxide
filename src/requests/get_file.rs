use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::File,
};

/// Use this method to get basic info about a file and prepare it for
/// downloading. For the moment, bots can download files of up to 20MB in size.
/// On success, a File object is returned.
/// The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>,
/// where <file_path> is taken from the response.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling getFile again.
#[derive(Debug, Clone, Serialize)]
pub struct GetFile<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// File identifier to get info about
    pub file_id: String,
}

#[async_trait]
impl<'a> Request<'a> for GetFile<'a> {
    type ReturnValue = File;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue>
    where
        Self: 'a
    {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "getFile",
            &self,
        )
        .await
    }
}

impl<'a> GetFile<'a> {
    pub(crate) fn new(ctx: RequestContext<'a>, file_id: String) -> Self {
        Self { ctx, file_id }
    }

    pub fn file_id<T>(mut self, file_id: T) -> Self
    where
        T: Into<String>,
    {
        self.file_id = file_id.into();
        self
    }
}
