use gloo_net::http::Request;
use poster_gen_core::{ClientRequest, Job, PeopleConfig, RemixParams};
use serde::Deserialize;

#[derive(Deserialize)]
struct ImageResponse {
    image: Option<String>,
    error: Option<String>,
}

pub async fn post_job(path: &str, key: &str, request: &ClientRequest) -> Result<String, String> {
    let response = Request::post(path)
        .header("Authorization", &format!("Bearer {key}"))
        .json(request)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;
    let status = response.status();
    let body: ImageResponse = response.json().await.map_err(|err| err.to_string())?;
    if !(200..300).contains(&status) {
        return Err(body.error.unwrap_or_else(|| format!("HTTP {status}")));
    }
    body.image.ok_or_else(|| "No image generated".to_string())
}

pub fn portrait_request(provider: poster_gen_core::ProviderId, model: String, config: PeopleConfig) -> ClientRequest {
    ClientRequest {
        provider,
        model,
        job: Job::Portrait { config },
    }
}

pub fn remix_request(
    provider: poster_gen_core::ProviderId,
    model: String,
    params: RemixParams,
) -> ClientRequest {
    ClientRequest {
        provider,
        model,
        job: Job::Remix { params },
    }
}
