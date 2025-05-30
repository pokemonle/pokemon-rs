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
#[sea_orm(table_name = "berries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub item_id: i32,
    pub firmness_id: i32,
    pub natural_gift_power: i32,
    pub natural_gift_type_id: i32,
    pub size: i32,
    pub max_harvest: i32,
    pub growth_time: i32,
    pub soil_dryness: i32,
    pub smoothness: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::berry_firmness::Entity",
        from = "Column::FirmnessId",
        to = "super::berry_firmness::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    BerryFirmness,
}

impl Related<super::berry_firmness::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BerryFirmness.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
