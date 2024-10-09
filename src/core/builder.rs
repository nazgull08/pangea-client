use lazy_static::lazy_static;

use super::{client::Client, error::Result, provider::Provider};

lazy_static! {
    static ref USERNAME: String = std::env::var("PANGEA_USERNAME").unwrap_or_default();
}
lazy_static! {
    static ref PASSWORD: String = std::env::var("PANGEA_PASSWORD").unwrap_or_default();
}
lazy_static! {
    static ref DEFAULT_ENDPOINT: String =
        std::env::var("PANGEA_URL").unwrap_or_else(|_| "app.pangea.foundation".to_string());
}

/// A builder for `Client`.
/// examples:
/// ```
/// use pangea_client::ClientBuilder;
/// use pangea_client::HttpProvider;
///
/// let client = ClientBuilder::default()
///   .endpoint("app.pangea.foundation")
///   .build::<HttpProvider>();
/// ```
///
/// ```
/// use pangea_client::ClientBuilder;
/// use pangea_client::WsProvider;
///
/// let client = ClientBuilder::default()
///  .endpoint("app.pangea.foundation")
///  .build::<WsProvider>();
/// ```
pub struct ClientBuilder {
    endpoint: String,
    is_secure: bool,
    username: Option<String>,
    password: Option<String>,
}

/// A builder for `Client`.
/// example:
/// ```
/// use pangea_client::ClientBuilder;
/// use pangea_client::HttpProvider;
///
/// let client = ClientBuilder::default()
///    .endpoint("app.pangea.foundation")
///    .credential("username", "password")
///   .build::<HttpProvider>();
/// ```
impl ClientBuilder {
    /// Sets the endpoint of the client.
    pub fn endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    /// Sets the username of the client.
    pub fn credential(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Sets the secure of the client.
    /// If the secure is false, the client will use http instead of https for
    /// API. Default is true.
    pub fn secure(mut self, is_secure: bool) -> Self {
        self.is_secure = is_secure;
        self
    }

    /// Creates a new `Client` with the given configuration.
    pub async fn build<T>(self) -> Result<Client<T>>
    where
        T: Provider + Send,
    {
        let inner = T::try_new(self.endpoint, self.is_secure, self.username, self.password).await?;
        Ok(Client::new(inner))
    }
}

/// Default implementation for `ClientBuilder`.
/// Default endpoint is `app.pangea.foundation`.
/// Default secure is true.
/// Default username is None.
/// Default password is None.
/// ```
/// use pangea_client::ClientBuilder;
/// use pangea_client::HttpProvider;
///
/// let client = ClientBuilder::default()
///   .build::<HttpProvider>();
/// ```
impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            username: Some(USERNAME.to_string()),
            password: Some(PASSWORD.to_string()),
            endpoint: DEFAULT_ENDPOINT.to_string(),
            is_secure: true,
        }
    }
}
