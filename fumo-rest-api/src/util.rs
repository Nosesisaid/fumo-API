use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};

// pub fn authentication_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode>{
//     let auth_header = req.headers().get(http::header::AUTHORIZATION).and_then(f);
// }

pub fn is_valid_api_token(
    header_to_check: Option<TypedHeader<Authorization<Bearer>>>,
    api_tokens: &Vec<String>,
) -> bool {
    let Some(header) = header_to_check else {
        return false;
    };

    if api_tokens.contains(&header.token().to_string()) {
        return true;
    }
    false
}
