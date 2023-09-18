use axum::{http::StatusCode, response::Json, Extension};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::db_serviciotec::tecnicos; // Asegúrate de importar correctamente el módulo de la entidad "Tecnicos"

#[derive(Deserialize)]
pub struct SolicitudTecnico {
    nombres: String,
    apaterno: String,
    amaterno: String,
    direccion: String,
    telefono: String,
    email: String,
    usuario: String,
    password: String,
}

#[derive(Serialize)]
pub struct RespuestaTecnico {
    id: i32,
    usuario: String,
    email: Option<String>,
}

// Crear un nuevo técnico
pub async fn tecnicos_create(
    Extension(dbconnection): Extension<DatabaseConnection>,
    Json(solicitud): Json<SolicitudTecnico>,
) -> Result<Json<RespuestaTecnico>, StatusCode> {
    let nuevo_tecnico = tecnicos::ActiveModel {
        nombres: Set(Some(solicitud.nombres)),
        apaterno: Set(Some(solicitud.apaterno)),
        amaterno: Set(Some(solicitud.amaterno)),
        direccion: Set(Some(solicitud.direccion)),
        telefono: Set(Some(solicitud.telefono)),
        email: Set(Some(solicitud.email)),
        usuario: Set(solicitud.usuario),
        password: Set(Some(solicitud.password)),
        ..Default::default()
    };

    let create_result = nuevo_tecnico.save(&dbconnection).await;

    match create_result {
        Ok(created_tecnico) => {
            let respuesta_tecnico = RespuestaTecnico {
                id: created_tecnico.id.unwrap(),
                usuario: created_tecnico.usuario.unwrap(),
                email: created_tecnico.email.unwrap(),
            };

            Ok(Json(respuesta_tecnico))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}