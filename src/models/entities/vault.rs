use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::entities::service;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "vault")]
pub struct Model {
#[sea_orm(primary_key)]
    pub id: Uuid,
    pub service_name: String,
    pub service_user_name: String,
    pub service_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Service,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Service => Entity::has_many(service::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
