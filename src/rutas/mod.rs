pub mod clientes {
    pub mod clientes_atomic_update;
    pub mod clientes_create;
    pub mod clientes_delete;
    pub mod clientes_list;
    pub mod clientes_partial_update;
}
pub mod tecnicos {

    pub mod tecnicos_create;
}
use axum::{
    routing::{delete, get, patch, post, put},
    Extension, Router,
};
use sea_orm::DatabaseConnection;

use clientes::{
    clientes_atomic_update::cliente_atomic_update,
    clientes_create::cliente_create,
    clientes_delete::cliente_soft_delete,
    clientes_list::{cliente_list_id, clientes_list_all},
    clientes_partial_update::cliente_partial_update,
};
use tecnicos::tecnicos_create::tecnicos_create;

pub async fn routes_handler(dbconnection: DatabaseConnection) -> Router {
    Router::new()
        .nest(
            "/clientes",
            Router::new()
                .route("/create", post(cliente_create))
                .route("/all", get(clientes_list_all))
                .route("/:id", get(cliente_list_id))
                .route("/:id", put(cliente_atomic_update))
                .route("/:id", patch(cliente_partial_update))
                .route("/:id", delete(cliente_soft_delete)),
        )
        .nest(
            "/tecnicos",
            Router::new().route("/create", post(tecnicos_create)),
        )
        .layer(Extension(dbconnection))
}
