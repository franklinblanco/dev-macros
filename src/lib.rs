mod auth;

/*#[proc_macro_attribute]
pub fn authenticated_route(_: TokenStream, mut input: TokenStream) -> TokenStream {    
    let tt_to_add = TokenStream::from(quote::quote!{
        let header_conversion_result: dev_dtos::dtos::user::user_dtos::UserAuthHeader = match serde_json::from_str(match request.headers().get("authentication") {
            Some(auth_header) => match auth_header.to_str() {
                Ok(string) => string,
                Err(_) => return actix_web_utils::extensions::typed_response::TypedHttpResponse::return_standard_error(400, err::MessageResource::new_from_str("Auth header in incorrect format.")),
            },
            None => return actix_web_utils::extensions::typed_response::TypedHttpResponse::return_standard_error(401, err::MessageResource::new_from_str("No auth header present.")),
        }) {
            Ok(user_for_authentication) => user_for_authentication,
            Err(error) => return actix_web_utils::extensions::typed_response::TypedHttpResponse::return_standard_error(401, err::MessageResource::new_from_string(error.to_string())),
        };
        let authenticated_user = match dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token(&client, &header_conversion_result.into()).await {
            Ok(authed_user) => authed_user,
            Err(error) => return actix_web_utils::extensions::typed_response::TypedHttpResponse::return_standard_error(401, err::MessageResource::new_from_string(error.to_string())),
        };
    });
    input.extend(tt_to_add);
    input
}*/

#[allow(unused_macros)]
#[macro_export]
macro_rules! authenticate_route {
    () => {
        let header_conversion_result: dev_dtos::dtos::user::user_dtos::UserAuthHeader = match serde_json::from_str(match request.headers().get("authentication") {
            Some(auth_header) => match auth_header.to_str() {
                Ok(string) => string,
                Err(_) => return actix_web_utils::extensions::typed_response::TypedHttpResponse::return_standard_error(400, err::MessageResource::new_from_str("Auth header in incorrect format.")),
            },
            None => return actix_web_utils::extensions::typed_response::TypedHttpResponse::return_standard_error(401, err::MessageResource::new_from_str("No auth header present.")),
        }) {
            Ok(user_for_authentication) => user_for_authentication,
            Err(error) => return actix_web_utils::extensions::typed_response::TypedHttpResponse::return_standard_error(401, err::MessageResource::new_from_string(error.to_string())),
        };
        let authenticated_user = match dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token(&client, &header_conversion_result.into()).await {
            Ok(authed_user) => authed_user,
            Err(error) => return actix_web_utils::extensions::typed_response::TypedHttpResponse::return_standard_error(401, err::MessageResource::new_from_string(error.to_string())),
        };
    };
}