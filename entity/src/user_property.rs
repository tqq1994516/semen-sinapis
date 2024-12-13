//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_property")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    #[sea_orm(column_type = "Blob")]
    pub password: Vec<u8>,
    pub alias: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub extra: Option<Json>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}