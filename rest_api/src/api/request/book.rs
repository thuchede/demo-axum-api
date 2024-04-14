// use chrono::NaiveDate;
// use sea_orm::DeriveIntoActiveModel;
// use entity::book::ActiveModel;
// use serde::{Deserialize, Serialize};
//
// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveIntoActiveModel)]
// pub struct BookCreateRequest {
//     pub name: String,
//     pub description: String,
//     pub release_date: NaiveDate,
//     pub author: String,
//     pub isbn: String,
// }