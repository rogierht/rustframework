mod db_serviciotec;
mod rutas;

use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let dbconection = Database::connect(database_uri).await.unwrap();
    let app = rutas::routes_handler(dbconection).await;
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
