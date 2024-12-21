use serde::{Deserialize, Serialize};

/// The struct holding the required parameters to do an authentication request for receiving
/// a new access token.
#[derive(Serialize)]
pub struct TokenRequest {
    /// OAuth client id which was assigned to the application / developer.
    pub client_id: String,
    /// OAuth client secret (matching the `client_id`) which was assigned to the application / developer.
    pub client_secret: String,
    /// Should be always `password`.
    pub grant_type: String,
    /// The username of the account which should be accessed.
    pub username: String,
    /// The password of the account which should be accessed.
    pub password: String,
}

/// The struct defining the successful response for a request for an access token.
#[derive(Deserialize)]
pub struct TokenResponse {
    /// The access token for all further requests.
    pub access_token: String,
    /// The type of the access token (should always be `bearer`).
    pub token_type: String,
    /// The refresh token which can be used to get a new access token.
    pub refresh_token: String,
    /// The token lifetime in remaining seconds.
    pub expires_in: i64,
    /// The scope for which the token can be used.
    pub scope: String,
    /// The customer number the token is valid for.
    #[serde(rename = "kdnr")]
    pub customer_number: String,
    /// An internal identification number.
    pub bpid: i64,
    /// An internal identification number.
    #[serde(rename = "kontaktId")]
    pub kontakt_id: i64,
}
