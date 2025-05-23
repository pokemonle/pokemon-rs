//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.11

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
    aide :: OperationIo,
)]
#[sea_orm(table_name = "location_names")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub location_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub local_language_id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::languages::Entity",
        from = "Column::LocalLanguageId",
        to = "super::languages::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Languages,
    #[sea_orm(
        belongs_to = "super::locations::Entity",
        from = "Column::LocationId",
        to = "super::locations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Locations,
}

impl Related<super::languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Languages.def()
    }
}

impl Related<super::locations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Locations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
