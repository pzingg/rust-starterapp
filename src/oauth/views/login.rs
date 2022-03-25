use jelly::actix_session::UserSession;
use jelly::actix_web::web;
use jelly::error::OAuthError;
use jelly::oauth;
use jelly::prelude::*;
use jelly::Result;
use jelly::SESSION_OAUTH_FLOW;

use crate::oauth::forms::LoginForm;

/// The login form.
pub async fn form(request: HttpRequest, path: web::Path<String>) -> Result<HttpResponse> {
    // if request.is_authenticated()? {
    //    return request.redirect("/dashboard/");
    // }

    let mut provider = path.into_inner();
    if !oauth::client::valid_provider(&provider) {
        provider = "google".to_string();
    }
    let form = LoginForm { provider: provider, ..Default::default() };

    request.get_session().remove(SESSION_OAUTH_FLOW);
    request.render(200, "oauth/login.html", {
        let mut ctx = Context::new();
        ctx.insert("form", &form);
        ctx
    })
}

/// POST-handler for logging in.
pub async fn authenticate(request: HttpRequest, form: web::Form<LoginForm>) -> Result<HttpResponse> {
    // if request.is_authenticated()? {
    //    return request.redirect("/dashboard/");
    // }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "oauth/login.html", {
            let mut context = Context::new();
            context.insert("error", "Invalid email.");
            context.insert("form", &form);
            context
        });
    }

    request_authorization(request, &form.provider, &form.email)
}

fn request_authorization(request: HttpRequest, provider: &str, email: &str) -> Result<HttpResponse> {
    match oauth::client::client_for(provider) {
        Some(client) => {
            let (authorization_request, pkce_code_verifier) = oauth::pkce_authorization_request(
                &client,
                Some(email),
            );
            let (authorize_url, csrf_token) = authorization_request.url();
            let flow = oauth::OAuthFlow {
                authorization_code: String::new(),
                provider_name: provider.to_owned(),
                csrf_token_secret: csrf_token.secret().into(),
                pkce_verifier_secret: pkce_code_verifier.secret().into(),
            };

            request.get_session().set(SESSION_OAUTH_FLOW, flow)?;
            request.redirect(&authorize_url.to_string())
        },
        _ => Err(OAuthError::RegisterProviderError(provider.to_owned()).into()),
    }
}
