use poster_gen_core::{
    build_upstream, data_url_from_b64, parse_bearer, redact, ClientRequest, CoreError, Job,
    KeyPlacement, UpstreamRequest,
};
use serde_json::{json, Value};
use worker::*;

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let path = req.path();
    if req.method() != Method::Post || (path != "/api/generate-model" && path != "/api/remix") {
        return json_error(404, "Not found");
    }

    let header = match req.headers().get("Authorization") {
        Ok(Some(value)) => value,
        _ => return json_error(400, CoreError::EmptyCredential.message()),
    };
    let key = match parse_bearer(&header) {
        Ok(key) => key,
        Err(err) => return json_error(400, err.message()),
    };

    let body: ClientRequest = match req.json().await {
        Ok(body) => body,
        Err(_) => return json_error(400, "请求格式无效"),
    };
    let route_ok = matches!(
        (path.as_str(), &body.job),
        ("/api/generate-model", Job::Portrait { .. }) | ("/api/remix", Job::Remix { .. })
    );
    if !route_ok {
        return json_error(400, "请求与路径不一致");
    }

    let upstream = match build_upstream(&body) {
        Ok(upstream) => upstream,
        Err(err) => return json_error(400, err.message()),
    };

    match forward(&upstream, &key).await {
        Ok(image) => Response::from_json(&json!({ "image": image })),
        Err(message) => json_error(502, &redact(&message, &key)),
    }
}

async fn forward(upstream: &UpstreamRequest, key: &str) -> std::result::Result<String, String> {
    let body = serde_json::to_string(&upstream.body).map_err(|err| err.to_string())?;
    let headers = Headers::new();
    headers
        .set("Content-Type", "application/json")
        .map_err(|err| err.to_string())?;
    match upstream.key_placement {
        KeyPlacement::BearerAuthorization => {
            headers
                .set("Authorization", &format!("Bearer {key}"))
                .map_err(|err| err.to_string())?;
        }
        KeyPlacement::GoogleApiKey => {
            headers
                .set("x-goog-api-key", key)
                .map_err(|err| err.to_string())?;
        }
    }

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(wasm_bindgen::JsValue::from_str(&body)));
    let request = Request::new_with_init(&upstream.url, &init).map_err(|err| err.to_string())?;
    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|err| err.to_string())?;
    let status = response.status_code();
    let text = response.text().await.map_err(|err| err.to_string())?;
    if !(200..300).contains(&status) {
        return Err(upstream_error(status, &text));
    }
    extract_image(&text).ok_or_else(|| "No image generated".to_string())
}

fn upstream_error(status: u16, body: &str) -> String {
    let value: Value = serde_json::from_str(body).unwrap_or(Value::Null);
    let message = value
        .pointer("/error/message")
        .and_then(Value::as_str)
        .or_else(|| value.get("error").and_then(Value::as_str))
        .unwrap_or("上游图像服务返回错误");
    format!("{message} (HTTP {status})")
}

fn extract_image(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    if let Some(b64) = value.pointer("/data/0/b64_json").and_then(Value::as_str) {
        return Some(data_url_from_b64(b64));
    }
    let parts = value.pointer("/candidates/0/content/parts")?.as_array()?;
    for part in parts {
        if let Some(data) = part
            .pointer("/inlineData/data")
            .or_else(|| part.pointer("/inline_data/data"))
            .and_then(Value::as_str)
        {
            if !data.is_empty() {
                return Some(data_url_from_b64(data));
            }
        }
    }
    None
}

fn json_error(status: u16, message: &str) -> Result<Response> {
    Ok(Response::from_json(&json!({ "error": message }))?.with_status(status))
}
