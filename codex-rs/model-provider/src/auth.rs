use std::sync::Arc;

use codex_agent_identity::AgentIdentityKey;
use codex_agent_identity::AgentTaskAuthorizationTarget;
use codex_agent_identity::authorization_header_for_agent_task;
use codex_api::AuthProvider;
use codex_api::SharedAuthProvider;
use codex_login::AuthManager;
use codex_login::CodexAuth;
use codex_model_provider_info::AMNEZIA_ROUTER_API_KEY_PREFIX;
use codex_model_provider_info::ModelProviderInfo;
use http::HeaderMap;
use http::HeaderValue;

use crate::bearer_auth_provider::BearerAuthProvider;

#[derive(Clone, Debug)]
struct AgentIdentityAuthProvider {
    auth: codex_login::auth::AgentIdentityAuth,
}

impl AuthProvider for AgentIdentityAuthProvider {
    fn add_auth_headers(&self, headers: &mut HeaderMap) {
        let record = self.auth.record();
        let header_value = authorization_header_for_agent_task(
            AgentIdentityKey {
                agent_runtime_id: &record.agent_runtime_id,
                private_key_pkcs8_base64: &record.agent_private_key,
            },
            AgentTaskAuthorizationTarget {
                agent_runtime_id: &record.agent_runtime_id,
                task_id: self.auth.process_task_id(),
            },
        )
        .map_err(std::io::Error::other);

        if let Ok(header_value) = header_value
            && let Ok(header) = HeaderValue::from_str(&header_value)
        {
            let _ = headers.insert(http::header::AUTHORIZATION, header);
        }

        if let Ok(header) = HeaderValue::from_str(self.auth.account_id()) {
            let _ = headers.insert("ChatGPT-Account-ID", header);
        }

        if self.auth.is_fedramp_account() {
            let _ = headers.insert("X-OpenAI-Fedramp", HeaderValue::from_static("true"));
        }
    }
}

// Some providers are meant to send no auth headers. Examples include local OSS
// providers and custom test providers with `requires_openai_auth = false`.
#[derive(Clone, Debug)]
struct UnauthenticatedAuthProvider;

impl AuthProvider for UnauthenticatedAuthProvider {
    fn add_auth_headers(&self, _headers: &mut HeaderMap) {}
}

pub fn unauthenticated_auth_provider() -> SharedAuthProvider {
    Arc::new(UnauthenticatedAuthProvider)
}

/// Returns the provider-scoped auth manager when this provider uses command-backed auth.
///
/// Providers without custom auth continue using the caller-supplied base manager, when present.
pub(crate) fn auth_manager_for_provider(
    auth_manager: Option<Arc<AuthManager>>,
    provider: &ModelProviderInfo,
) -> Option<Arc<AuthManager>> {
    match provider.auth.clone() {
        Some(config) => Some(AuthManager::external_bearer_only(config)),
        None => auth_manager,
    }
}

pub(crate) fn resolve_provider_auth(
    auth: Option<&CodexAuth>,
    provider: &ModelProviderInfo,
) -> codex_protocol::error::Result<SharedAuthProvider> {
    if provider.is_amnezia_router() {
        return amnezia_router_auth(auth);
    }

    if let Some(auth) = bearer_auth_for_provider(provider)? {
        return Ok(Arc::new(auth));
    }

    Ok(match auth {
        Some(auth) => auth_provider_from_auth(auth),
        None => unauthenticated_auth_provider(),
    })
}

fn amnezia_router_auth(
    auth: Option<&CodexAuth>,
) -> codex_protocol::error::Result<SharedAuthProvider> {
    let Some(auth) = auth else {
        return Ok(unauthenticated_auth_provider());
    };
    let token = auth.get_token().map_err(|err| {
        codex_protocol::error::CodexErr::InvalidRequest(format!(
            "Amnezia Router auth token is unavailable: {err}"
        ))
    })?;
    if !token.starts_with(AMNEZIA_ROUTER_API_KEY_PREFIX) {
        return Err(codex_protocol::error::CodexErr::InvalidRequest(
            "Amnezia Router refuses non-router credentials; run configured OIDC login to exchange credentials for an agpt_pat_ router key.".to_string(),
        ));
    }
    Ok(Arc::new(BearerAuthProvider::new(token)))
}

pub(crate) fn ensure_auth_manager_matches_provider(
    auth_manager: Option<&AuthManager>,
    provider: &ModelProviderInfo,
) -> codex_protocol::error::Result<()> {
    if provider.is_amnezia_router()
        && !auth_manager.is_some_and(AuthManager::uses_configured_oidc_auth)
    {
        return Err(codex_protocol::error::CodexErr::InvalidRequest(
            "Amnezia Router backend requires configured OIDC auth; refusing to send legacy OpenAI/ChatGPT credentials to the router.".to_string(),
        ));
    }
    Ok(())
}

fn bearer_auth_for_provider(
    provider: &ModelProviderInfo,
) -> codex_protocol::error::Result<Option<BearerAuthProvider>> {
    if let Some(api_key) = provider.api_key()? {
        return Ok(Some(BearerAuthProvider::new(api_key)));
    }

    if let Some(token) = provider.experimental_bearer_token.clone() {
        return Ok(Some(BearerAuthProvider::new(token)));
    }

    Ok(None)
}

/// Builds request-header auth for a first-party Codex auth snapshot.
pub fn auth_provider_from_auth(auth: &CodexAuth) -> SharedAuthProvider {
    match auth {
        CodexAuth::AgentIdentity(auth) => {
            Arc::new(AgentIdentityAuthProvider { auth: auth.clone() })
        }
        CodexAuth::ApiKey(_) | CodexAuth::Chatgpt(_) | CodexAuth::ChatgptAuthTokens(_) => {
            Arc::new(BearerAuthProvider {
                token: auth.get_token().ok(),
                account_id: auth.get_account_id(),
                is_fedramp_account: auth.is_fedramp_account(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use codex_model_provider_info::AMNEZIA_ROUTER_API_KEY_PREFIX;
    use codex_model_provider_info::WireApi;
    use codex_model_provider_info::create_oss_provider_with_base_url;

    use super::*;

    #[test]
    fn unauthenticated_auth_provider_adds_no_headers() {
        let provider =
            create_oss_provider_with_base_url("http://localhost:11434/v1", WireApi::Responses);
        let auth = resolve_provider_auth(/*auth*/ None, &provider).expect("auth should resolve");

        assert!(auth.to_auth_headers().is_empty());
    }

    #[test]
    fn amnezia_router_rejects_non_router_bearer_token() {
        let provider = ModelProviderInfo::create_amnezia_router_provider(/*base_url*/ None);
        let auth = CodexAuth::from_api_key("sk-openai");

        let err = match resolve_provider_auth(Some(&auth), &provider) {
            Ok(_) => panic!("router should reject non-router credentials"),
            Err(err) => err,
        };

        assert!(
            err.to_string().contains("non-router credentials"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn amnezia_router_accepts_router_bearer_token() {
        let provider = ModelProviderInfo::create_amnezia_router_provider(/*base_url*/ None);
        let auth = CodexAuth::from_api_key(&format!("{AMNEZIA_ROUTER_API_KEY_PREFIX}test"));

        let auth_provider =
            resolve_provider_auth(Some(&auth), &provider).expect("router key should resolve");

        let headers = auth_provider.to_auth_headers();
        assert_eq!(
            headers
                .get(http::header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok()),
            Some("Bearer agpt_pat_test")
        );
    }
}
