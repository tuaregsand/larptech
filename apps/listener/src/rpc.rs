use async_stream::stream;
use futures_util::{SinkExt, Stream, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Debug, Deserialize, Clone)]
pub struct RawLog {
    pub logs: Vec<String>,
    pub slot: u64,
    pub program_id: String,
}

#[derive(Debug, Deserialize)]
struct RpcResponse {
    pub params: RpcParams,
}

#[derive(Debug, Deserialize)]
struct RpcParams {
    pub result: RpcResult,
}

#[derive(Debug, Deserialize)]
struct RpcResult {
    pub context: Context,
    pub value: RpcValue,
}

#[derive(Debug, Deserialize)]
struct Context {
    pub slot: u64,
}

#[derive(Debug, Deserialize)]
struct RpcValue {
    pub logs: Vec<String>,
    pub program_id: String,
}

pub async fn connect_and_stream(
    url: &str,
) -> Result<impl Stream<Item = RawLog>, Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, read) = ws_stream.split();

    let sub = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            { "mentions": ["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"] },
            { "commitment": "confirmed" }
        ]
    });
    write.send(Message::Text(sub.to_string())).await?;

    Ok(stream! {
        let mut read = read;
        while let Some(msg) = read.next().await {
            if let Ok(Message::Text(text)) = msg {
                if let Ok(resp) = serde_json::from_str::<RpcResponse>(&text) {
                    let result = resp.params.result;
                    let raw = RawLog {
                        logs: result.value.logs,
                        slot: result.context.slot,
                        program_id: result.value.program_id,
                    };
                    yield raw;
                }
            }
        }
    })
}
