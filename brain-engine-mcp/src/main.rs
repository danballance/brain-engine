use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

#[derive(Debug, Deserialize)]
struct SumParams {
    a: i64,
    b: i64,
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Failed to parse request: {}", e);
                continue;
            }
        };

        let response = handle_request(request);
        let response_json = serde_json::to_string(&response)?;
        writeln!(stdout, "{}", response_json)?;
        stdout.flush()?;
    }

    Ok(())
}

fn handle_request(request: JsonRpcRequest) -> JsonRpcResponse {
    let result = match request.method.as_str() {
        "initialize" => handle_initialize(),
        "tools/list" => handle_tools_list(),
        "tools/call" => handle_tool_call(request.params),
        _ => Err(JsonRpcError {
            code: -32601,
            message: format!("Method not found: {}", request.method),
        }),
    };

    match result {
        Ok(value) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(value),
            error: None,
        },
        Err(error) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(error),
        },
    }
}

fn handle_initialize() -> Result<Value, JsonRpcError> {
    Ok(json!({
        "protocolVersion": "2024-11-05",
        "serverInfo": {
            "name": "brain-engine-mcp",
            "version": "0.1.0"
        },
        "capabilities": {
            "tools": {}
        }
    }))
}

fn handle_tools_list() -> Result<Value, JsonRpcError> {
    Ok(json!({
        "tools": [
            {
                "name": "sum",
                "description": "Add two integers together",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "a": {
                            "type": "number",
                            "description": "First number"
                        },
                        "b": {
                            "type": "number",
                            "description": "Second number"
                        }
                    },
                    "required": ["a", "b"]
                }
            }
        ]
    }))
}

fn handle_tool_call(params: Option<Value>) -> Result<Value, JsonRpcError> {
    let params = params.ok_or_else(|| JsonRpcError {
        code: -32602,
        message: "Missing params".to_string(),
    })?;

    let tool_name = params.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing tool name".to_string(),
        })?;

    match tool_name {
        "sum" => {
            let arguments = params.get("arguments").ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing arguments".to_string(),
            })?;

            let sum_params: SumParams = serde_json::from_value(arguments.clone())
                .map_err(|e| JsonRpcError {
                    code: -32602,
                    message: format!("Invalid arguments: {}", e),
                })?;

            let result = sum_params.a + sum_params.b;

            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": format!("The sum of {} and {} is {}", sum_params.a, sum_params.b, result)
                    }
                ]
            }))
        }
        _ => Err(JsonRpcError {
            code: -32602,
            message: format!("Unknown tool: {}", tool_name),
        }),
    }
}
