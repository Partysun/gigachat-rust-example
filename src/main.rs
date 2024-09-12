use anyhow::Ok;
use async_gigachat::{
    chat::{Chat, ChatCompletionRequestBuilder, ChatMessage, Role},
    client::Client,
    config::GigaChatConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    panic!("\nProvide your GIGACHAT_API_CORP TOKEN\nOpen and edit main.rs file\n");
    // Remove the above line after
    let auth_token = "";

    let config = GigaChatConfig::builder()
        .auth_token(auth_token)
        .scope("GIGACHAT_API_CORP")
        .build();

    let client = Client::with_config(config);

    let request = ChatCompletionRequestBuilder::default()
        .messages(vec![ChatMessage {
            role: Some(Role::User),
            content: "Hey, how's it going?".into(),
        }])
        .model("GigaChat:latest")
        .build()
        .unwrap();

    let response = Chat::new(client).completion(request).await.unwrap();

    println!("{}", response.choices.get(0).unwrap().message.content);
    Ok(())
}
