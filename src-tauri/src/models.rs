#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;
use serde::{ Serialize, Deserialize };
use crate::schema::*;

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable, Selectable)]
#[diesel(table_name = assignments)]
pub struct Assignment {
    pub id: i32,
    pub name: String,
    pub assessment_type: String,
    pub unit: String,
    pub submitted: i32,
}