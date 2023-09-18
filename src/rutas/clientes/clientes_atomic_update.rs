use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::db_serviciotec::clientes;
use crate::db_serviciotec::clientes::Entity as Clientes;

#[derive(Deserialize)]
pub struct RespuestaCliente {
    pub id: Option<i32>,
    pub nombres: Option<String>,
    pub apaterno: Option<String>,
    pub amaterno: Option<String>,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub eliminado_en: Option<chrono::NaiveDateTime>,
}

pub async fn cliente_atomic_update(
    Path(cliente_id): Path<i32>,
    Extension(dbconnection): Extension<DatabaseConnection>,
    Json(respuesta_cliente): Json<RespuestaCliente>,
) -> Result<(), StatusCode> {
    let update_cli = clientes::ActiveModel {
        id: Set(cliente_id),
        nombres: Set(respuesta_cliente.nombres),
        apaterno: Set(respuesta_cliente.apaterno),
        amaterno: Set(respuesta_cliente.amaterno),
        direccion: Set(respuesta_cliente.direccion),
        telefono: Set(respuesta_cliente.telefono),
        email: Set(respuesta_cliente.email),
        eliminado_en: Set(respuesta_cliente.eliminado_en),
    };

    let update_result = Clientes::update(update_cli)
        .filter(clientes::Column::Id.eq(cliente_id))
        .exec(&dbconnection)
        .await;

    match update_result {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}