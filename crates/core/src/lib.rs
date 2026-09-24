//! Provider-neutral rules for a Poster remix.
//!
//! This crate never sees a Credential. The Worker attaches the caller's key
//! when it forwards the request.

mod prompt;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub use prompt::{portrait_prompt, remix_prompt};

pub const GROK_DEFAULT_MODEL: &str = "grok-imagine-image-quality";
pub const GEMINI_DEFAULT_MODEL: &str = "gemini-3-pro-image-preview";
pub const XAI_ORIGIN: &str = "https://api.x.ai";
pub const GEMINI_ORIGIN: &str = "https://generativelanguage.googleapis.com";

/// Longest data URL we will forward. Compressed uploads stay well under this.
const MAX_DATA_URL_LEN: usize = 7_000_000;

const ASPECT_RATIOS: &[&str] = &["1:1", "16:9", "9:16", "4:3", "3:4"];
const RESOLUTIONS: &[&str] = &["1K", "2K"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderId {
    Grok,
    Gemini,
}

impl ProviderId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Grok => "grok",
            Self::Gemini => "gemini",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Grok => "xAI Grok Imagine",
            Self::Gemini => "Google Gemini",
        }
    }

    pub fn default_model(self) -> &'static str {
        match self {
            Self::Grok => GROK_DEFAULT_MODEL,
            Self::Gemini => GEMINI_DEFAULT_MODEL,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    EmptyModel,
    InvalidModel,
    MissingBase,
    MissingReference,
    InvalidImage,
    ImageTooLarge,
    InvalidSetting,
    EmptyCredential,
    InvalidCredential,
}

impl CoreError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::EmptyModel => "模型 id 不能为空",
            Self::InvalidModel => "模型 id 含有不允许的字符",
            Self::MissingBase => "请先上传基础图",
            Self::MissingReference => "请至少提供一项替换——产品图、上传的人物，或生成的模特",
            Self::InvalidImage => "图片必须是浏览器里的 data URL",
            Self::ImageTooLarge => "图片太大，无法发送",
            Self::InvalidSetting => "宽高比或分辨率不受支持",
            Self::EmptyCredential => "请先填写当前模型的密钥",
            Self::InvalidCredential => "密钥格式无效",
        }
    }
}

/// A Remix may run only when a base image and at least one reference are present.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemixGate {
    Ready,
    MissingBase,
    MissingReference,
}

pub fn remix_gate(has_base: bool, has_product: bool, has_people: bool) -> RemixGate {
    if !has_base {
        RemixGate::MissingBase
    } else if !(has_product || has_people) {
        RemixGate::MissingReference
    } else {
        RemixGate::Ready
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeopleConfig {
    pub gender: String,
    pub hair_color: String,
    pub skin_tone: String,
    pub ethnicity: String,
    pub expression: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemixParams {
    pub base_image: String,
    pub product_image: Option<String>,
    pub people_image: Option<String>,
    pub style_description: String,
    pub custom_prompt: String,
    pub aspect_ratio: String,
    pub resolution: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind")]
pub enum Job {
    Portrait { config: PeopleConfig },
    Remix { params: RemixParams },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRequest {
    pub provider: ProviderId,
    pub model: String,
    pub job: Job,
}

/// Where the Worker should place the caller's key. The key is not in `body`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPlacement {
    BearerAuthorization,
    GoogleApiKey,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpstreamRequest {
    pub url: String,
    pub body: Value,
    pub key_placement: KeyPlacement,
}

pub fn build_upstream(request: &ClientRequest) -> Result<UpstreamRequest, CoreError> {
    let model = validate_model_id(&request.model)?;
    match (&request.provider, &request.job) {
        (ProviderId::Grok, Job::Portrait { config }) => Ok(UpstreamRequest {
            url: format!("{XAI_ORIGIN}/v1/images/generations"),
            body: json!({
                "model": model,
                "prompt": portrait_prompt(config),
                "aspect_ratio": "3:4",
                "resolution": "1k",
                "response_format": "b64_json",
                "n": 1
            }),
            key_placement: KeyPlacement::BearerAuthorization,
        }),
        (ProviderId::Grok, Job::Remix { params }) => {
            let params = validate_remix(params)?;
            let (prompt, images) = grok_remix_parts(&params);
            Ok(UpstreamRequest {
                url: format!("{XAI_ORIGIN}/v1/images/edits"),
                body: json!({
                    "model": model,
                    "prompt": prompt,
                    "images": images,
                    "aspect_ratio": params.aspect_ratio,
                    "response_format": "b64_json",
                    "n": 1
                }),
                key_placement: KeyPlacement::BearerAuthorization,
            })
        }
        (ProviderId::Gemini, Job::Portrait { config }) => Ok(UpstreamRequest {
            url: gemini_url(model),
            body: json!({
                "contents": [{
                    "parts": [{ "text": portrait_prompt(config) }]
                }],
                "generationConfig": {
                    "imageConfig": {
                        "aspectRatio": "3:4",
                        "imageSize": "1K"
                    }
                }
            }),
            key_placement: KeyPlacement::GoogleApiKey,
        }),
        (ProviderId::Gemini, Job::Remix { params }) => {
            let params = validate_remix(params)?;
            Ok(UpstreamRequest {
                url: gemini_url(model),
                body: json!({
                    "contents": [{ "parts": gemini_remix_parts(&params) }],
                    "generationConfig": {
                        "imageConfig": {
                            "aspectRatio": params.aspect_ratio,
                            "imageSize": params.resolution
                        }
                    }
                }),
                key_placement: KeyPlacement::GoogleApiKey,
            })
        }
    }
}

fn gemini_url(model: &str) -> String {
    format!("{GEMINI_ORIGIN}/v1beta/models/{model}:generateContent")
}

fn validate_model_id(model: &str) -> Result<&str, CoreError> {
    let model = model.trim();
    if model.is_empty() {
        return Err(CoreError::EmptyModel);
    }
    if model.len() > 128
        || !model
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
    {
        return Err(CoreError::InvalidModel);
    }
    Ok(model)
}

fn validate_remix(params: &RemixParams) -> Result<RemixParams, CoreError> {
    let product = blank_to_none(params.product_image.as_deref());
    let people = blank_to_none(params.people_image.as_deref());
    let has_base = !params.base_image.trim().is_empty();
    match remix_gate(has_base, product.is_some(), people.is_some()) {
        RemixGate::MissingBase => return Err(CoreError::MissingBase),
        RemixGate::MissingReference => return Err(CoreError::MissingReference),
        RemixGate::Ready => {}
    }
    require_data_image(&params.base_image)?;
    if let Some(image) = &product {
        require_data_image(image)?;
    }
    if let Some(image) = &people {
        require_data_image(image)?;
    }
    if !ASPECT_RATIOS.contains(&params.aspect_ratio.as_str())
        || !RESOLUTIONS.contains(&params.resolution.as_str())
    {
        return Err(CoreError::InvalidSetting);
    }
    Ok(RemixParams {
        base_image: params.base_image.clone(),
        product_image: product,
        people_image: people,
        style_description: params.style_description.clone(),
        custom_prompt: params.custom_prompt.clone(),
        aspect_ratio: params.aspect_ratio.clone(),
        resolution: params.resolution.clone(),
    })
}

fn blank_to_none(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

pub fn require_data_image(url: &str) -> Result<(), CoreError> {
    if url.len() > MAX_DATA_URL_LEN {
        return Err(CoreError::ImageTooLarge);
    }
    let Some(rest) = url.strip_prefix("data:image/") else {
        return Err(CoreError::InvalidImage);
    };
    if !rest.contains(";base64,") {
        return Err(CoreError::InvalidImage);
    }
    Ok(())
}

/// Split a data URL into `(mime, base64)`.
pub fn split_data_url(url: &str) -> Result<(String, String), CoreError> {
    require_data_image(url)?;
    let (header, data) = url
        .split_once(',')
        .ok_or(CoreError::InvalidImage)?;
    let mime = header
        .strip_prefix("data:")
        .and_then(|h| h.split(';').next())
        .filter(|m| !m.is_empty())
        .ok_or(CoreError::InvalidImage)?;
    if data.is_empty() {
        return Err(CoreError::InvalidImage);
    }
    Ok((mime.to_string(), data.to_string()))
}

fn grok_remix_parts(params: &RemixParams) -> (String, Vec<Value>) {
    let mut images = vec![json!({ "type": "image_url", "url": params.base_image })];
    let mut lines = vec![
        "Image 1 is the BASE IMAGE that must be remixed into a new commercial advertisement."
            .to_string(),
    ];
    let mut n = 2;
    if let Some(product) = &params.product_image {
        images.push(json!({ "type": "image_url", "url": product }));
        lines.push(format!(
            "Image {n} is a PRODUCT reference: REPLACE the main product/object in the base image with this object. Ensure the lighting and perspective match the new scene."
        ));
        n += 1;
    }
    if let Some(people) = &params.people_image {
        images.push(json!({ "type": "image_url", "url": people }));
        lines.push(format!(
            "Image {n} is a PEOPLE reference: REPLACE the people in the base image with this person/people. Ensure realistic blending, consistent lighting, and match the pose/vibe of the base image where appropriate."
        ));
    }
    let prompt = format!(
        "{}\n{}",
        lines.join("\n"),
        remix_prompt(&params.style_description, &params.custom_prompt)
    );
    (prompt, images)
}

fn gemini_remix_parts(params: &RemixParams) -> Vec<Value> {
    let mut parts = vec![
        json!({ "text": "Here is the BASE IMAGE that I want to remix into a new commercial advertisement:" }),
        inline_part(&params.base_image),
    ];
    if let Some(product) = &params.product_image {
        parts.push(json!({
            "text": "Use the object in the following REFERENCE IMAGE to REPLACE the main product/object in the base image. Ensure the lighting and perspective match the new scene:"
        }));
        parts.push(inline_part(product));
    }
    if let Some(people) = &params.people_image {
        parts.push(json!({
            "text": "Use the person/people in the following REFERENCE IMAGE to REPLACE the people in the base image. Ensure realistic blending, consistent lighting, and match the pose/vibe of the base image where appropriate:"
        }));
        parts.push(inline_part(people));
    }
    parts.push(json!({
        "text": remix_prompt(&params.style_description, &params.custom_prompt)
    }));
    parts
}

fn inline_part(data_url: &str) -> Value {
    let (mime, data) = split_data_url(data_url).expect("image already validated");
    json!({ "inlineData": { "mimeType": mime, "data": data } })
}

/// Pull the key out of an `Authorization: Bearer …` header.
pub fn parse_bearer(header: &str) -> Result<String, CoreError> {
    let header = header.trim();
    let key = if header.eq_ignore_ascii_case("bearer") {
        ""
    } else {
        header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
            .unwrap_or(header)
            .trim()
    };
    if key.is_empty() {
        return Err(CoreError::EmptyCredential);
    }
    if key.chars().any(char::is_control) || key.contains(' ') {
        return Err(CoreError::InvalidCredential);
    }
    Ok(key.to_string())
}

pub fn redact(message: &str, secret: &str) -> String {
    if secret.is_empty() {
        message.to_string()
    } else {
        message.replace(secret, "[redacted]")
    }
}

pub fn data_url_from_b64(b64: &str) -> String {
    format!("data:image/png;base64,{b64}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png() -> String {
        "data:image/png;base64,aaaa".to_string()
    }

    fn remix(product: bool, people: bool) -> RemixParams {
        RemixParams {
            base_image: png(),
            product_image: product.then(png),
            people_image: people.then(png),
            style_description: "minimal".into(),
            custom_prompt: "".into(),
            aspect_ratio: "1:1".into(),
            resolution: "1K".into(),
        }
    }

    #[test]
    fn base_alone_is_not_a_remix() {
        assert_eq!(remix_gate(true, false, false), RemixGate::MissingReference);
        assert_eq!(remix_gate(false, true, true), RemixGate::MissingBase);
        assert_eq!(remix_gate(true, true, false), RemixGate::Ready);
        assert_eq!(remix_gate(true, false, true), RemixGate::Ready);
    }

    #[test]
    fn grok_portrait_matches_the_previous_request_shape() {
        let req = ClientRequest {
            provider: ProviderId::Grok,
            model: format!("  {GROK_DEFAULT_MODEL}  "),
            job: Job::Portrait {
                config: PeopleConfig {
                    gender: "Female".into(),
                    hair_color: "Brown".into(),
                    skin_tone: "Medium".into(),
                    ethnicity: "Mixed".into(),
                    expression: "Neutral".into(),
                },
            },
        };
        let up = build_upstream(&req).unwrap();
        assert_eq!(up.url, "https://api.x.ai/v1/images/generations");
        assert_eq!(up.key_placement, KeyPlacement::BearerAuthorization);
        assert_eq!(up.body["model"], GROK_DEFAULT_MODEL);
        assert_eq!(up.body["aspect_ratio"], "3:4");
        assert_eq!(up.body["resolution"], "1k");
        assert_eq!(up.body["response_format"], "b64_json");
        assert!(up.body["prompt"].as_str().unwrap().contains("Female"));
        assert!(!up.body.to_string().contains("secret-key"));
    }

    #[test]
    fn grok_edit_omits_resolution_and_numbers_images() {
        let req = ClientRequest {
            provider: ProviderId::Grok,
            model: GROK_DEFAULT_MODEL.into(),
            job: Job::Remix {
                params: remix(true, true),
            },
        };
        let up = build_upstream(&req).unwrap();
        assert_eq!(up.url, "https://api.x.ai/v1/images/edits");
        assert!(up.body.get("resolution").is_none());
        assert_eq!(up.body["images"].as_array().unwrap().len(), 3);
        let prompt = up.body["prompt"].as_str().unwrap();
        assert!(prompt.contains("Image 1 is the BASE IMAGE"));
        assert!(prompt.contains("Image 2 is a PRODUCT"));
        assert!(prompt.contains("Image 3 is a PEOPLE"));
        assert!(prompt.contains("DO NOT generate any text"));
    }

    #[test]
    fn custom_prompt_allows_typography() {
        let mut params = remix(true, false);
        params.custom_prompt = "  put the brand name in the corner  ".into();
        let req = ClientRequest {
            provider: ProviderId::Gemini,
            model: GEMINI_DEFAULT_MODEL.into(),
            job: Job::Remix { params },
        };
        let up = build_upstream(&req).unwrap();
        let body = up.body.to_string();
        assert!(body.contains("put the brand name"));
        assert!(!body.contains("DO NOT generate any text"));
        assert_eq!(up.key_placement, KeyPlacement::GoogleApiKey);
        assert!(up.url.starts_with(GEMINI_ORIGIN));
        assert!(up.url.contains(GEMINI_DEFAULT_MODEL));
        assert_eq!(up.body["generationConfig"]["imageConfig"]["aspectRatio"], "1:1");
        assert_eq!(up.body["generationConfig"]["imageConfig"]["imageSize"], "1K");
    }

    #[test]
    fn remix_without_a_reference_is_rejected() {
        let req = ClientRequest {
            provider: ProviderId::Gemini,
            model: GEMINI_DEFAULT_MODEL.into(),
            job: Job::Remix {
                params: remix(false, false),
            },
        };
        assert_eq!(build_upstream(&req).unwrap_err(), CoreError::MissingReference);
    }

    #[test]
    fn remote_urls_and_model_injection_are_rejected() {
        let mut params = remix(true, false);
        params.product_image = Some("https://example.com/p.png".into());
        let req = ClientRequest {
            provider: ProviderId::Grok,
            model: GROK_DEFAULT_MODEL.into(),
            job: Job::Remix { params },
        };
        assert_eq!(build_upstream(&req).unwrap_err(), CoreError::InvalidImage);

        let sneaky = ClientRequest {
            provider: ProviderId::Gemini,
            model: "../other:generateContent".into(),
            job: Job::Portrait {
                config: PeopleConfig {
                    gender: "Female".into(),
                    hair_color: "Black".into(),
                    skin_tone: "Fair".into(),
                    ethnicity: "Asian".into(),
                    expression: "Neutral".into(),
                },
            },
        };
        assert_eq!(build_upstream(&sneaky).unwrap_err(), CoreError::InvalidModel);
    }

    #[test]
    fn bearer_and_redaction() {
        assert_eq!(parse_bearer("Bearer abc.def").unwrap(), "abc.def");
        assert_eq!(parse_bearer("  bearer abc ").unwrap(), "abc");
        assert!(parse_bearer("Bearer ").is_err());
        assert!(parse_bearer("Bearer abc\nX-Evil: 1").is_err());
        assert_eq!(
            redact("upstream said abc.def failed", "abc.def"),
            "upstream said [redacted] failed"
        );
    }
}
