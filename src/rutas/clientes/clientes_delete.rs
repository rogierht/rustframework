use axum::extract::Query;
use axum::http::StatusCode;
use axum::{extract::Path, Extension};
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::Deserialize;

use crate::db_serviciotec::clientes;
use crate::db_serviciotec::clientes::Entity as Clientes;

#[derive(Deserialize)]
pub struct ParametrosConsulta {
    soft: bool,
}

/* pub async fn cliente_soft_delete(
    Path(cliente_id): Path<i32>,
    Extension(dbconnection): Extension<DatabaseConnection>,
    Query(parametros_consulta): Query<ParametrosConsulta>,
) -> Result<(), StatusCode> {
    if parametros_consulta.soft {
        let mut client = if let Some(client) = Clientes::find_by_id(cliente_id)
            .one(&dbconnection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            client.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        let fecha_actual: DateTime<Utc> = Utc::now();
        let fecha_actual_naive: NaiveDateTime = fecha_actual.naive_utc();
        client.eliminado_en = Set(Some(fecha_actual_naive));

        Clientes::update(client)
            .exec(&dbconnection)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Clientes::delete_many()
            .filter(clientes::Column::Id.eq(cliente_id))
            .exec(&dbconnection)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(())
} */

/* pub async fn cliente_delete(
    Path(cliente_id): Path<i32>,
    Extension(dbconnection): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    match Clientes::delete_many()
        .filter(clientes::Column::Id.eq(cliente_id))
        .exec(&dbconnection)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
} */

pub async fn cliente_soft_delete(
    Path(cliente_id): Path<i32>, // Parámetro de ruta: ID del cliente a eliminar
    Extension(dbconnection): Extension<DatabaseConnection>, // Extensión para acceder a la conexión de la base de datos
    Query(parametros_consulta): Query<ParametrosConsulta>,  // Parámetros de consulta opcionales
) -> Result<(), StatusCode> {
    if parametros_consulta.soft == true {
        // Si se proporciona el parámetro "soft" y tiene un valor verdadero
        let mut client = if let Some(client) = Clientes::find_by_id(cliente_id)
            .one(&dbconnection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?// Manejo de error: si no se puede realizar la consulta, se devuelve un error 500
        {
            // Convertir el cliente obtenido en un modelo activo para editar sus atributos
            client.into_active_model() 
        } else {
            // Si no se encontró el cliente, devolver un error 404
            return Err(StatusCode::NOT_FOUND);
        };

        // Obtener la fecha y hora actual en formato UTC
        let fecha_actual: DateTime<Utc> = Utc::now(); 

        // Convertir la fecha y hora actual a un formato naive DateTime
        let fecha_actual_naive: NaiveDateTime = fecha_actual.naive_utc();

        // Establecer el campo "eliminado_en" del cliente con la fecha y hora actual
        client.eliminado_en = Set(Some(fecha_actual_naive)); 

        Clientes::update(client)
            .exec(&dbconnection)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?; // Actualizar el cliente en la base de datos y manejar el error en caso de que ocurra
    } else {
        // Si no se proporciona el parámetro "soft" o tiene un valor falso
        Clientes::delete_many()
            .filter(clientes::Column::Id.eq(cliente_id))
            .exec(&dbconnection)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?; // Eliminar el cliente de la base de datos y manejar el error en caso de que ocurra
    }
    // Devolver un resultado exitoso sin valor
    Ok(())
}