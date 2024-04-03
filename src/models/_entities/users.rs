//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pid: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    #[sea_orm(unique)]
    pub api_key: String,
    pub first_name: String,
    pub last_name: String,
    pub fiscal_code: Option<String>,
    pub phone_number: Option<String>,
    pub birth_place: Option<String>,
    pub residence: Option<String>,
    pub color: Option<String>,
    pub image_url: Option<String>,
    pub birth_date: Option<Date>,
    pub reset_token: Option<String>,
    pub reset_sent_at: Option<DateTime>,
    pub email_verification_token: Option<String>,
    pub email_verification_sent_at: Option<DateTime>,
    pub email_verified_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
