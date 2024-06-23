use axum::extract::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use chrono::Utc;
use tracing::error;
use crate::bo::user::{
    CreateUserRequestBo, CreateUserResponseBo, GetUserResponseBo, UserSideRsaKeyPairBo,
};
use crate::encryption::generate_rsa_key_pair;
pub async fn create_user(
    Json(create_user_request): Json<CreateUserRequestBo>,
) -> Result<Json<CreateUserResponseBo>, StatusCode> {
    let user_name = create_user_request.user_name;
    let user_name_md5 = md5::compute(user_name);
    let user_name_md5 = format!("{user_name_md5:x}");
    let agent_rsa_key_pair = tokio::spawn(async move {
        generate_rsa_key_pair().map_err(|e| {
            error!("Fail to generate rsa key pair for agent because of error: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
    });
    let proxy_rsa_key_pair = tokio::spawn(async move {
        generate_rsa_key_pair().map_err(|e| {
            error!("Fail to generate rsa key pair for proxy because of error: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
    });
    let (agent_rsa_key_pair, proxy_rsa_key_pair) =
        tokio::join!(agent_rsa_key_pair, proxy_rsa_key_pair);
    let user_side_rsa_key_pair = UserSideRsaKeyPairBo {
        agent_private_key: agent_rsa_key_pair
            .map_err(|e| {
                error!("Fail to execute generate agent rsa key pair task because of error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })??
            .private_key_pem,
        proxy_public_key: proxy_rsa_key_pair
            .map_err(|e| {
                error!("Fail to execute generate proxy rsa key pair task because of error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })??
            .public_key_pem,
    };
    Ok(Json(CreateUserResponseBo {
        user_side_rsa_key_pair,
    }))
}

pub async fn get_user(
    Path(user_name): Path<String>,
) -> Result<Json<GetUserResponseBo>, StatusCode> {
    let register_timestamp = Utc::now();
    let user_side_rsa_key_pair = UserSideRsaKeyPairBo {
        agent_private_key: "".to_string(),
        proxy_public_key: "".to_string(),
    };
    Ok(Json(GetUserResponseBo {
        user_name,
        user_side_rsa_key_pair,
        register_timestamp,
    }))
}
