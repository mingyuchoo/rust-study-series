use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Axum SQLx Postgres API", version = "0.1.0"),
    paths(
        crate::adapters::http::routes::user::register,
        crate::adapters::http::routes::user::list_users,
        crate::adapters::http::routes::user::get_user,
        crate::adapters::http::routes::user::update_user,
        crate::adapters::http::routes::user::delete_user
    ),
    components(
        schemas(
            crate::adapters::http::routes::user::RegisterPayload,
            crate::adapters::http::routes::user::RegisterResponse,
            crate::adapters::http::routes::user::UserListItemResponse,
            crate::adapters::http::routes::user::UpdatePayload,
            crate::adapters::http::routes::user::UpdateResponse,
            crate::adapters::http::routes::user::DeleteResponse
        )
    ),
    tags(
        (name = "user", description = "User management endpoints")
    )
)]
pub struct ApiDoc;
