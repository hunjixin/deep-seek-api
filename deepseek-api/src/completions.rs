use crate::{
    error::ToApiError,
    json_stream::JsonStream,
    request::{
        CompletionsRequestBuilder, FMICompletionsRequestBuilder, MessageRequest, RequestBuilder,
    },
    response::ChatResponse,
};
use anyhow::Result;
use reqwest::Client as ReqwestClient;
pub struct ChatCompletions {
    pub(crate) client: ReqwestClient,
    pub(crate) host: &'static str,
}

impl ChatCompletions {
    pub fn chat_builder(&self, messages: Vec<MessageRequest>) -> CompletionsRequestBuilder {
        CompletionsRequestBuilder::new(messages)
    }

    pub fn fim_builder(&self, prompt: &str, suffix: &str) -> FMICompletionsRequestBuilder {
        FMICompletionsRequestBuilder::new(prompt, suffix)
    }

    pub async fn create<Builder>(
        &mut self,
        request_builder: Builder,
    ) -> Result<ChatResponse<Builder::Response, Builder::Item>>
    where
        Builder: RequestBuilder + Send,
    {
        let host = if request_builder.is_beta() {
            self.host.to_owned() + "/beta/completions"
        } else {
            self.host.to_owned() + "/chat/completions"
        };
        let is_stream = request_builder.is_stream();

        let request = request_builder.build();
        let resp = self
            .client
            .post(&host)
            .json(&request)
            .send()
            .await?
            .to_api_err()
            .await?;
        if is_stream {
            Ok(ChatResponse::Stream(JsonStream::new(resp)))
        } else {
            Ok(ChatResponse::Full(resp.json().await?))
        }
    }
}
