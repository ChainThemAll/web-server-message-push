use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use oso::Oso;
use server_oso::role::Role;
use std::convert::Infallible;
use std::sync::Arc;
async fn handle_request(req: Request<Body>, oso: Arc<Oso>) -> Result<Response<Body>, Infallible> {
    // Assume the role is provided in a header for simplicity
    let role_header = req
        .headers()
        .get("Role")
        .and_then(|value| value.to_str().ok());

    let user_role = match role_header {
        Some("admin") => Role::Administrator,
        Some("user") => Role::User,
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::empty())
                .unwrap())
        }
    };

    let user = User {
        name: String::from("Alice"), // This would typically come from the request as well
        role: user_role.clone(),
    };

    let is_allowed = oso.is_allowed(user, "read", "data").unwrap();

    if is_allowed {
        let response_body = match user_role {
            Role::Administrator => "admin",
            Role::User => "user",
            // ... other roles
        };
        Ok(Response::new(Body::from(response_body)))
    } else {
        Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::empty())
            .unwrap())
    }
}
