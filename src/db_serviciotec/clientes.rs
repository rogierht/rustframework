//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "clientes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nombres: Option<String>,
    pub apaterno: Option<String>,
    pub amaterno: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub eliminado_en: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::diagnosticos::Entity")]
    Diagnosticos,
    #[sea_orm(has_many = "super::equipos::Entity")]
    Equipos,
}

impl Related<super::diagnosticos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Diagnosticos.def()
    }
}

impl Related<super::equipos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Equipos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}