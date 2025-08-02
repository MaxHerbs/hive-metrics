use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tracing::info;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tokio::time::interval;


static JWK_CACHE: OnceCell<Arc<RwLock<Arc<JwkSet>>>> = OnceCell::new();

fn init_jwk_cache() -> Arc<RwLock<Arc<JwkSet>>> {
    JWK_CACHE
        .get_or_init(|| Arc::new(RwLock::new(Arc::new(JwkSet { keys: vec![] }))))
        .clone()
}

const JWKS_URL: &str = "https://authn.theoffice.uk/realms/master/protocol/openid-connect/certs";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub email: Option<String>,
}

pub async fn jwk_refresh_loop() {
    let cache = init_jwk_cache();
    let mut interval = interval(Duration::from_secs(300));

    loop {
        interval.tick().await;
        match fetch_jwk_set().await {
            Ok(new_jwks) => {
                let mut guard = cache.write().await;
                *guard = Arc::new(new_jwks);
                println!("[JWK Refresh] Success");
            }
            Err(e) => eprintln!("[JWK Refresh] Error: {}", e),
        }
    }
}

async fn fetch_jwk_set() -> anyhow::Result<JwkSet> {
    let resp = reqwest::get(JWKS_URL).await?;
    let jwks: JwkSet = resp.json().await?;
    Ok(jwks)
}

pub async fn validate_jwt_from_header(headers: &HeaderMap) -> anyhow::Result<Claims> {
    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| anyhow::anyhow!("Missing Authorization header"))?
        .to_str()?;
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or("Invalid Authorization scheme").unwrap();

    let header = decode_header(token)?;
    let kid = header.kid.ok_or("Missing kid").unwrap();


    let cache = init_jwk_cache();

    let jwks = cache.read().await;
    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.common.key_id.as_deref() == Some(&kid))
        .ok_or(anyhow::anyhow!("JWK not found"))?;

    let decoding_key = DecodingKey::from_jwk(jwk)?;
    let mut validation = Validation::new(header.alg);
    validation.set_audience(&["proxy"]);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}
