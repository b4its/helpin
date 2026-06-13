use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use super::auth_middleware::AuthUser;

/// Role-based access control extractor.
///
/// Use `RequireRole::<"admin", "supplier">` pattern or use the helper `RequireRoles` struct
/// to restrict handler access to specific roles.
///
/// # Usage
/// ```ignore
/// async fn admin_only(RequireRoles(auth): RequireRoles<{ &["admin"] }>) -> impl IntoResponse { ... }
/// ```
///
/// For a simpler approach, use the `require_roles` helper function in handlers:
/// ```ignore
/// async fn handler(auth: AuthUser) -> Result<impl IntoResponse, RbacError> {
///     require_roles(&auth, &["admin", "supplier"])?;
///     // handler logic
/// }
/// ```
#[derive(Debug)]
pub struct RbacError {
    pub required_roles: Vec<String>,
    pub actual_role: String,
}

impl IntoResponse for RbacError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": "Forbidden",
            "message": format!(
                "Role '{}' is not authorized. Required: {:?}",
                self.actual_role, self.required_roles
            ),
        }));

        (StatusCode::FORBIDDEN, body).into_response()
    }
}

/// Helper function to check if the authenticated user has one of the allowed roles.
/// Use this in handlers after extracting AuthUser.
///
/// # Example
/// ```ignore
/// async fn handler(auth: AuthUser) -> Result<Json<Value>, RbacError> {
///     require_roles(&auth, &["admin", "peternak"])?;
///     Ok(Json(json!({"ok": true})))
/// }
/// ```
pub fn require_roles(auth: &AuthUser, allowed_roles: &[&str]) -> Result<(), RbacError> {
    if allowed_roles.contains(&auth.role.as_str()) {
        Ok(())
    } else {
        Err(RbacError {
            required_roles: allowed_roles.iter().map(|s| s.to_string()).collect(),
            actual_role: auth.role.clone(),
        })
    }
}

/// An extractor that combines AuthUser extraction with role checking.
/// Provide the allowed roles at construction time.
///
/// # Usage in route handlers:
/// ```ignore
/// // In your router setup:
/// let require_admin = RequireRoles::new(vec!["admin".to_string()]);
///
/// // Or use the helper directly in handlers (recommended):
/// async fn my_handler(auth: AuthUser) -> Result<impl IntoResponse, RbacError> {
///     require_roles(&auth, &["admin"])?;
///     // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct RequireRoles {
    pub allowed_roles: Vec<String>,
    pub auth: AuthUser,
}

impl RequireRoles {
    /// Create role requirements for use with the extractor pattern.
    pub fn roles(allowed: &[&str]) -> AllowedRoles {
        AllowedRoles {
            roles: allowed.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// Configuration struct placed in request extensions to define which roles are allowed.
#[derive(Debug, Clone)]
pub struct AllowedRoles {
    pub roles: Vec<String>,
}

impl<S> FromRequestParts<S> for RequireRoles
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // First extract AuthUser
        let auth = AuthUser::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;

        // Check if AllowedRoles is configured in extensions
        let allowed = parts.extensions.get::<AllowedRoles>().cloned();

        if let Some(allowed_roles) = allowed {
            if !allowed_roles.roles.contains(&auth.role) {
                let err = RbacError {
                    required_roles: allowed_roles.roles,
                    actual_role: auth.role,
                };
                return Err(err.into_response());
            }

            Ok(RequireRoles {
                allowed_roles: allowed_roles.roles,
                auth,
            })
        } else {
            // If no AllowedRoles configured, just pass through with auth
            Ok(RequireRoles {
                allowed_roles: vec![],
                auth,
            })
        }
    }
}
