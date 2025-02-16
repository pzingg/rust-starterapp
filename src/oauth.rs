//! OAuth2 authentication.

use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};

pub mod forms;
pub mod views;

/// Enables oauth2 login and authentication.
pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/oauth")
            .service(
                resource("/login/{provider}")
                    .route(get().to(views::login::form)),
            )
            .service(
                resource("/login")
                    .route(post().to(views::login::authenticate)),
            )
            .service(
                resource("/callback")
                    .name("oauth-callback")
                    .route(get().to(views::authorize::exchange_code_for_token)),
            )
            .service(
                resource("/confirm")
                    .route(post().to(views::authorize::confirm_identity)),
            ),
    );
}
