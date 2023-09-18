use axum::{extract::Path, http::StatusCode, Extension, Json};
use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::Serialize;

use crate::db_serviciotec::clientes::{Entity as Clientes, self};

#[derive(Serialize)]
pub struct RespuestaCliente {
    id: i32,
    nombres: Option<String>,
    apaterno: Option<String>,
    amaterno: Option<String>,
    direccion: Option<String>,
    telefono: Option<String>,
    email: Option<String>,
    eliminado_en: Option<NaiveDateTime>,
}

// listar todos los clientes
pub async fn clientes_list_all(
    Extension(dbconnection): Extension<DatabaseConnection>,
) -> Result<Json<Vec<RespuestaCliente>>, StatusCode> {
    let clientes = Clientes::find()
        .filter(clientes::Column::EliminadoEn.is_null())
        .all(&dbconnection)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    let respuesta_clientes: Vec<RespuestaCliente> = clientes
        .into_iter()
        .map(|cliente| RespuestaCliente {
            id: cliente.id,
            nombres: cliente.nombres,
            apaterno: cliente.apaterno,
            amaterno: cliente.amaterno,
            direccion: cliente.direccion,
            telefono: cliente.telefono,
            email: cliente.email,
            eliminado_en: cliente.eliminado_en,
        })
        .collect();

    if respuesta_clientes.is_empty() {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(Json(respuesta_clientes))
    }
}

// listar cliente por ID
pub async fn cliente_list_id(
    Path(cliente_id): Path<i32>,
    Extension(dbconnection): Extension<DatabaseConnection>,
) -> Result<Json<RespuestaCliente>, (StatusCode, String)> {
    let cli = Clientes::find_by_id(cliente_id)
        .filter(clientes::Column::EliminadoEn.is_null())
        .one(&dbconnection)
        .await
        .unwrap();
    if let Some(cliente) = cli {
        Ok(Json(RespuestaCliente {
            id: cliente.id,
            nombres: cliente.nombres,
            apaterno: cliente.apaterno,
            amaterno: cliente.amaterno,
            direccion: cliente.direccion,
            telefono: cliente.telefono,
            email: cliente.email,
            eliminado_en: cliente.eliminado_en,
        }))
    } else {
        let error_message = "No se encontró el ID";
        Err((StatusCode::NOT_FOUND, error_message.to_string()))
    }
}