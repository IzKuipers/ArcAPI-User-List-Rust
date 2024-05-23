use crate::types::{
    info::{dummy_server_info, ServerInfo},
    user::{dummy_user_info, ApiResponse},
};

pub fn get_user_list(hostname: String, authcode: String, https: bool, port: u16) -> ApiResponse {
    let protocol = if https { "https://" } else { "http://" };
    let url = format!("{protocol}{hostname}:{port}/users/get?ac={authcode}");
    let call = reqwest::blocking::get(url);

    return match call {
        Ok(data) => {
            let response = data.text().unwrap_or_default();
            let json: ApiResponse =
                serde_json::from_str(response.as_str()).unwrap_or(dummy_user_info());

            return json;
        }
        Err(_) => dummy_user_info(),
    };
}

pub fn get_server_info(hostname: String, authcode: String, https: bool, port: u16) -> ServerInfo {
    let protocol = if https { "https://" } else { "http://" };
    let url = format!("{protocol}{hostname}:{port}/v2/?ac={authcode}");
    let call = reqwest::blocking::get(url);

    return match call {
        Ok(data) => {
            let response = data.text().unwrap_or_default();
            let json: ServerInfo =
                serde_json::from_str(response.as_str()).unwrap_or(dummy_server_info());

            return json;
        }
        Err(_) => dummy_server_info(),
    };
}
