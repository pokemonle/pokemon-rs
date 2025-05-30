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
#[sea_orm(table_name = "types")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(unique)]
    pub identifier: String,
    pub generation_id: i32,
    pub damage_class_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::generations::Entity",
        from = "Column::GenerationId",
        to = "super::generations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Generations,
    #[sea_orm(
        belongs_to = "super::move_damage_classes::Entity",
        from = "Column::DamageClassId",
        to = "super::move_damage_classes::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MoveDamageClasses,
    #[sea_orm(has_many = "super::moves::Entity")]
    Moves,
    #[sea_orm(has_many = "super::pokemon_evolution::Entity")]
    PokemonEvolution,
    #[sea_orm(has_many = "super::pokemon_types::Entity")]
    PokemonTypes,
    #[sea_orm(has_many = "super::type_names::Entity")]
    TypeNames,
}

impl Related<super::generations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Generations.def()
    }
}

impl Related<super::move_damage_classes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MoveDamageClasses.def()
    }
}

impl Related<super::moves::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Moves.def()
    }
}

impl Related<super::pokemon_evolution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PokemonEvolution.def()
    }
}

impl Related<super::pokemon_types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PokemonTypes.def()
    }
}

impl Related<super::type_names::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TypeNames.def()
    }
}

impl Related<super::languages::Entity> for Entity {
    fn to() -> RelationDef {
        super::type_names::Relation::Languages.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::type_names::Relation::Types.def().rev())
    }
}

impl Related<super::pokemon::Entity> for Entity {
    fn to() -> RelationDef {
        super::pokemon_types::Relation::Pokemon.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::pokemon_types::Relation::Types.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
