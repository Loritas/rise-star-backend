use axum::{
    http::StatusCode,
    extract::{Json, Query},
};
use axum::response::IntoResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    code: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    openid: String,
    session_key: String,
    msg: String,
}

pub async fn login_user_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    let appid = "wx4162d1cb0e588452";  // 替换成你的微信小程序 App ID
    let secret = "a82d65d11ece8d991cf0dedf4abbc724";  // 替换成你的微信小程序 App Secret

    let client = Client::new();
    let url = format!(
        "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
        appid, secret, payload.code
    );

    match client.get(&url).send().await {
        Ok(response) => {
            if let Ok(res_body) = response.json::<serde_json::Value>().await {
                if let (Some(openid), Some(session_key)) = (res_body.get("openid"), res_body.get("session_key")) {
                    (
                        StatusCode::OK,
                        Json(LoginResponse {
                            openid: openid.as_str().unwrap().to_string(),
                            session_key: session_key.as_str().unwrap().to_string(),
                            msg: "Login success".to_string(),
                        }),
                    )
                } else {
                    (StatusCode::BAD_REQUEST, Json(LoginResponse{
                        openid: "".to_string(),
                        session_key: "".to_string(),
                        msg: res_body.to_string(),
                    }))
                }
            } else {
                (StatusCode::BAD_REQUEST, Json(LoginResponse{
                    openid: "".to_string(),
                    session_key: "".to_string(),
                    msg: "Request failed".to_string(),
                }))
            }
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(LoginResponse{
            openid: "".to_string(),
            session_key: "".to_string(),
            msg: "Request failed".to_string(),
        }))
    }
}