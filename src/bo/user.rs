use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct CreateUserRequestBo {
    pub user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSideRsaKeyPairBo {
    pub agent_private_key: String,
    pub proxy_public_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponseBo {
    pub user_side_rsa_key_pair: UserSideRsaKeyPairBo,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserResponseBo {
    pub register_timestamp: DateTime<Utc>,
    pub user_name: String,
    pub user_side_rsa_key_pair: UserSideRsaKeyPairBo,
}
