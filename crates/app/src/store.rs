use poster_gen_core::ProviderId;
use serde::{Deserialize, Serialize};
use crate::copy::provider_storage_key;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub api_key: String,
    pub model_id: String,
}

impl Credential {
    pub fn fresh(provider: ProviderId) -> Self {
        Self {
            api_key: String::new(),
            model_id: provider.default_model().to_string(),
        }
    }
}

pub fn load_credential(provider: ProviderId) -> Credential {
    let Some(storage) = local_storage() else {
        return Credential::fresh(provider);
    };
    let Ok(Some(raw)) = storage.get_item(provider_storage_key(provider)) else {
        return Credential::fresh(provider);
    };
    serde_json::from_str::<Credential>(&raw).unwrap_or_else(|_| Credential::fresh(provider))
}

pub fn save_credential(provider: ProviderId, credential: &Credential) {
    let Some(storage) = local_storage() else {
        return;
    };
    if let Ok(raw) = serde_json::to_string(credential) {
        let _ = storage.set_item(provider_storage_key(provider), &raw);
    }
}

pub fn forget_credential(provider: ProviderId) {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item(provider_storage_key(provider));
    }
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}
