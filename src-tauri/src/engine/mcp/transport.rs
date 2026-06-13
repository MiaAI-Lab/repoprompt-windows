//! MCP transport server
//!
//! Provides a simple TCP-based server that speaks JSON-RPC for
//! the Model Context Protocol. External agents connect via TCP
//! and issue tool calls.

use std::sync::atomic::{AtomicBool, Ordering};

use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde_json::{json, Value};

/// Represents a JSON-RPC request
#[derive(Debug, serde::Deserialize)]
struct JsonRpcRequest {
    jsonrpc: Option<String>,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

/// Represents a JSON-RPC response
#[derive(Debug, serde::Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

#[derive(Debug, serde::Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

/// Server state
static SERVER_RUNNING: AtomicBool = AtomicBool::new(false);

/// Start the MCP TCP server on the given port
/// Returns Ok if the server starts successfully
pub async fn start_server(port: u16) -> Result<(), String> {
    if SERVER_RUNNING.load(Ordering::SeqCst) {
        return Err("MCP server is already running".to_string());
    }

    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

    SERVER_RUNNING.store(true, Ordering::SeqCst);

    // Spawn the accept loop
    tokio::spawn(async move {
        log::info!("MCP server listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    log::info!("MCP client connected from {}", peer_addr);
                    tokio::spawn(handle_client(stream));
                }
                Err(e) => {
                    log::error!("MCP accept error: {}", e);
                    break;
                }
            }
        }

        SERVER_RUNNING.store(false, Ordering::SeqCst);
    });

    Ok(())
}

/// Stop the MCP server
pub fn stop_server() {
    SERVER_RUNNING.store(false, Ordering::SeqCst);
    log::info!("MCP server stopped");
}

/// Check if the MCP server is running
pub fn is_running() -> bool {
    SERVER_RUNNING.load(Ordering::SeqCst)
}

/// Handle a single client connection
async fn handle_client(stream: tokio::net::TcpStream) {
    let (reader, mut writer) = stream.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        match buf_reader.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let response = match serde_json::from_str::<JsonRpcRequest>(trimmed) {
                    Ok(request) => handle_rpc_request(request).await,
                    Err(e) => JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32700,
                            message: format!("Parse error: {}", e),
                        }),
                        id: None,
                    },
                };

                let resp_json = serde_json::to_string(&response).unwrap_or_default();
                if let Err(e) = writer.write_all(format!("{}\n", resp_json).as_bytes()).await {
                    log::error!("Failed to write MCP response: {}", e);
                    break;
                }
            }
            Err(e) => {
                log::error!("MCP read error: {}", e);
                break;
            }
        }
    }
}

/// Handle a JSON-RPC request by dispatching to the appropriate tool
async fn handle_rpc_request(request: JsonRpcRequest) -> JsonRpcResponse {
    let method = &request.method;

    match method.as_str() {
        // MCP required methods
        "initialize" => JsonRpcResponse {
            jsonrpc: "2.0".into(),
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "reprompt-windows",
                    "version": "0.1.0"
                }
            })),
            error: None,
            id: request.id,
        },

        "tools/list" => {
            let tools = super::get_tools();
            JsonRpcResponse {
                jsonrpc: "2.0".into(),
                result: Some(json!({ "tools": tools })),
                error: None,
                id: request.id,
            }
        }

        "tools/call" => {
            if let Some(params) = request.params {
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let arguments = params.get("arguments");

                // For tool execution, we return a descriptive response since
                // actual execution requires the Tauri app context
                JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    result: Some(json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Tool '{}' recognized. Arguments: {:?}. Full execution requires Tauri app context.", tool_name, arguments)
                        }]
                    })),
                    error: None,
                    id: request.id,
                }
            } else {
                JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing parameters".into(),
                    }),
                    id: request.id,
                }
            }
        }

        "notifications/initialized" => JsonRpcResponse {
            jsonrpc: "2.0".into(),
            result: Some(json!({})),
            error: None,
            id: request.id,
        },

        _ => JsonRpcResponse {
            jsonrpc: "2.0".into(),
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", method),
            }),
            id: request.id,
        },
    }
}


