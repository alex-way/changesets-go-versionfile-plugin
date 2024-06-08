use plugin::{GetVersionResponse, SetVersionResponse};
use prost::Message;
use std::io;
use std::io::prelude::*;

pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/plugin.rs"));
}

pub fn deserialize_request(buf: &[u8]) -> Result<plugin::RequestMessage, prost::DecodeError> {
    plugin::RequestMessage::decode(buf)
}

pub fn serialize_response(resp: &plugin::Response) -> Vec<u8> {
    let mut buf = Vec::with_capacity(resp.encoded_len());

    resp.encode(&mut buf).unwrap();
    buf
}

pub fn handle_get_version_request(req: plugin::GetVersionRequest) -> plugin::Response {
    let mut file = match std::fs::File::open(req.file_path) {
        Ok(f) => f,
        Err(e) => {
            return plugin::Response {
                status: Some(plugin::Status {
                    code: 1,
                    message: format!("Error opening file: {}", e),
                }),
                response: Some(plugin::response::Response::GetVersion(GetVersionResponse {
                    version: "".to_string(),
                })),
            }
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return plugin::Response {
            status: Some(plugin::Status {
                code: 1,
                message: format!("Error reading file: {}", e),
            }),
            response: Some(plugin::response::Response::GetVersion(GetVersionResponse {
                version: "".to_string(),
            })),
        };
    }

    plugin::Response {
        status: Some(plugin::Status {
            code: 0,
            message: "".to_string(),
        }),
        response: Some(plugin::response::Response::GetVersion(GetVersionResponse {
            version: contents.trim().to_string(),
        })),
    }
}

fn handle_set_version_request(req: plugin::SetVersionRequest) -> plugin::Response {
    let mut file = match std::fs::File::create(req.file_path) {
        Ok(f) => f,
        Err(e) => {
            return plugin::Response {
                status: Some(plugin::Status {
                    code: 1,
                    message: format!("Error opening file: {}", e),
                }),
                response: Some(plugin::response::Response::SetVersion(
                    SetVersionResponse {},
                )),
            }
        }
    };

    if let Err(e) = file.write_all(req.version.as_bytes()) {
        return plugin::Response {
            status: Some(plugin::Status {
                code: 1,
                message: format!("Error writing file: {}", e),
            }),
            response: Some(plugin::response::Response::SetVersion(
                SetVersionResponse {},
            )),
        };
    }

    plugin::Response {
        status: Some(plugin::Status {
            code: 0,
            message: "".to_string(),
        }),
        response: Some(plugin::response::Response::SetVersion(
            SetVersionResponse {},
        )),
    }
}

fn main() -> Result<(), prost::DecodeError> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buffer = stdin.fill_buf().unwrap();

    let req = match deserialize_request(buffer) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(_e) => std::process::exit(1),
    };

    let resp = match req.request {
        Some(plugin::request_message::Request::GetVersion(req)) => handle_get_version_request(req),
        Some(plugin::request_message::Request::SetVersion(req)) => handle_set_version_request(req),
        None => plugin::Response {
            status: Some(plugin::Status {
                code: 1,
                message: "No request provided".to_string(),
            }),
            response: None,
        },
    };

    let out = serialize_response(&resp);

    match io::stdout().write_all(&out) {
        Ok(result) => result,
        Err(_e) => std::process::exit(1),
    };

    Ok(())
}
