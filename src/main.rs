use prost::Message;
use std::io;
use std::io::prelude::*;

pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/plugin.rs"));
}

pub fn deserialize_get_version_request(
    buf: &[u8],
) -> Result<plugin::GetVersionRequest, prost::DecodeError> {
    plugin::GetVersionRequest::decode(buf)
}

pub fn serialize_get_version_response(resp: &plugin::GetVersionResponse) -> Vec<u8> {
    let mut buf = Vec::with_capacity(resp.encoded_len());

    resp.encode(&mut buf).unwrap();
    buf
}

pub fn handle_get_version_request(req: plugin::GetVersionRequest) -> plugin::GetVersionResponse {
    let mut file = match std::fs::File::open(req.file_path) {
        Ok(f) => f,
        Err(e) => {
            return plugin::GetVersionResponse {
                version: "".to_string(),
                status: Some(plugin::Status {
                    code: 1,
                    message: format!("Error opening file: {}", e),
                }),
            }
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return plugin::GetVersionResponse {
            version: "".to_string(),
            status: Some(plugin::Status {
                code: 1,
                message: format!("Error reading file: {}", e),
            }),
        };
    }

    plugin::GetVersionResponse {
        version: contents.trim().to_string(),
        status: Some(plugin::Status {
            code: 0,
            message: "".to_string(),
        }),
    }
}

fn main() -> Result<(), prost::DecodeError> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buffer = stdin.fill_buf().unwrap();

    let req = match deserialize_get_version_request(buffer) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(_e) => std::process::exit(1),
    };

    let resp = handle_get_version_request(req);
    let out = serialize_get_version_response(&resp);

    match io::stdout().write_all(&out) {
        Ok(result) => result,
        Err(_e) => std::process::exit(1),
    };

    Ok(())
}
