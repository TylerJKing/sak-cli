use anyhow::{Context, Result};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    time::{Duration, SystemTime},
};
use url::Url;

use super::token_cache::{CachedToken, TokenCache};

const AUTH_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";
const TOKEN_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";
const REDIRECT_URL: &str = "http://localhost:8888/oauth/callback";
const SUCCESS_PAGE: &str = r#"
<!DOCTYPE html>
<html>
<head><title>Authentication Successful</title></head>
<body>
    <h1>Authentication Successful!</h1>
    <p>You can close this window and return to the CLI.</p>
</body>
</html>
"#;

pub struct InteractiveAuthFlow {
    client: BasicClient,
    token_cache: TokenCache,
}

impl InteractiveAuthFlow {
    pub fn new(client_id: &str, client_secret: Option<&str>) -> Result<Self> {
        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            client_secret.map(|s| ClientSecret::new(s.to_string())),
            AuthUrl::new(AUTH_URL.to_string())?,
            Some(TokenUrl::new(TOKEN_URL.to_string())?),
        )
        .set_redirect_uri(RedirectUrl::new(REDIRECT_URL.to_string())?);

        Ok(Self {
            client,
            token_cache: TokenCache::new()?,
        })
    }

    pub async fn get_token(&self) -> Result<String> {
        // Check cache first
        if let Some(token) = self.token_cache.get_token()? {
            return Ok(token.access_token);
        }

        // No valid cached token, start interactive flow
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_plain();

        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("User.Read".to_string()))
            .add_scope(Scope::new("Mail.Read".to_string()))
            .add_scope(Scope::new("Calendars.Read".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        // Start local server
        let listener = TcpListener::bind("127.0.0.1:8888").context("Failed to start local server")?;

        // Open browser
        webbrowser::open(auth_url.as_str())?;

        // Wait for the callback
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut reader = BufReader::new(&stream);
                let mut request_line = String::new();
                reader.read_line(&mut request_line)?;

                let redirect_url = request_line.split_whitespace().nth(1)
                    .ok_or_else(|| anyhow::anyhow!("Invalid request"))?;
                let url = Url::parse(&format!("http://localhost{}", redirect_url))?;

                // Write success page
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                    SUCCESS_PAGE.len(),
                    SUCCESS_PAGE
                );
                stream.write_all(response.as_bytes())?;

                // Extract the authorization code
                let code = url.query_pairs()
                    .find(|(key, _)| key == "code")
                    .map(|(_, value)| value.into_owned())
                    .ok_or_else(|| anyhow::anyhow!("No code in response"))?;

                let state = url.query_pairs()
                    .find(|(key, _)| key == "state")
                    .map(|(_, value)| value.into_owned())
                    .ok_or_else(|| anyhow::anyhow!("No state in response"))?;

                if state != csrf_token.secret() {
                    return Err(anyhow::anyhow!("CSRF token mismatch"));
                }

                // Exchange code for token
                let token = self.client
                    .exchange_code(AuthorizationCode::new(code))
                    .set_pkce_verifier(pkce_verifier)
                    .request_async(oauth2::reqwest::async_http_client)
                    .await?;

                // Cache the token
                let expires_at = SystemTime::now() + Duration::from_secs(3600); // Default to 1 hour
                let cached_token = CachedToken {
                    access_token: token.access_token().secret().clone(),
                    refresh_token: token.refresh_token()
                        .ok_or_else(|| anyhow::anyhow!("No refresh token"))?
                        .secret()
                        .clone(),
                    expires_at,
                };
                self.token_cache.save_token(&cached_token)?;

                return Ok(cached_token.access_token);
            }
        }

        Err(anyhow::anyhow!("Failed to get authorization code"))
    }

    pub fn clear_token_cache(&self) -> Result<()> {
        self.token_cache.clear()
    }
}