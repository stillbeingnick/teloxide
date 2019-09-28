use async_trait::async_trait;

use crate::{
    network,
    requests::{
        ChatId, Request, RequestContext, ResponseResult,
    },
};

///Use this method when you need to tell the user that something is happening
/// on the bot's side. The status is set for 5 seconds or less (when a message
/// arrives from your bot, Telegram clients clear its typing status).
/// Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct SendChatAction<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Type of action to broadcast. Choose one, depending on what the user is
    /// about to receive: typing for text messages, upload_photo for photos,
    /// record_video or upload_video for videos, record_audio or upload_audio
    /// for audio files, upload_document for general files, find_location for
    /// location data, record_video_note or upload_video_note for video notes.
    pub action: ChatAction,
}

#[derive(Debug, Serialize, From, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

#[async_trait]
impl<'a> Request<'a> for SendChatAction<'a> {
    type ReturnValue = bool;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue>
    where
        Self: 'a
    {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "sendChatAction",
            &self,
        )
        .await
    }
}

impl<'a> SendChatAction<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        action: ChatAction,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            action,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn action<T>(mut self, action: T) -> Self
    where
        T: Into<ChatAction>,
    {
        self.action = action.into();
        self
    }
}
