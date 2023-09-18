use axum::{http::StatusCode, response::Json, Extension};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::db_serviciotec::clientes;

#[derive(Deserialize)]
pub struct SolicitudCliente {
    nombres: String,
    apaterno: String,
    amaterno: String,
    direccion: String,
    telefono: String,
    email: String,
}

#[derive(Serialize)]
pub struct RespuestaCliente {
    id: i32,
    nombres: Option<String>,
    apaterno: Option<String>,
    amaterno: Option<String>,
}

// crear un nuevo cliente
pub async fn cliente_create(
    Extension(dbconnection): Extension<DatabaseConnection>,
    Json(solicitud): Json<SolicitudCliente>,
) -> Result<Json<RespuestaCliente>, StatusCode> {
    let nuevo_cliente = clientes::ActiveModel {
        nombres: Set(Some(solicitud.nombres)),
        apaterno: Set(Some(solicitud.apaterno)),
        amaterno: Set(Some(solicitud.amaterno)),
        direccion: Set(Some(solicitud.direccion)),
        telefono: Set(Some(solicitud.telefono)),
        email: Set(Some(solicitud.email)),
        ..Default::default()
    };

    let create_result = nuevo_cliente.save(&dbconnection).await;

    match create_result {
        Ok(created_cliente) => {
            let respuesta_cliente = RespuestaCliente {
                id: created_cliente.id.unwrap(),
                nombres: created_cliente.nombres.unwrap(),
                apaterno: created_cliente.apaterno.unwrap(),
                amaterno: created_cliente.amaterno.unwrap(),
            };

            Ok(Json(respuesta_cliente))
        }

        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
