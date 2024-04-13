use crate::settings::Settings;
use std::sync::Arc;
use arc_swap::ArcSwap;
use sea_orm::DatabaseConnection;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub db_conn: ArcSwap<DatabaseConnection>,
}

impl ApplicationState {
    pub fn new(settings: &Settings, db_conn: DatabaseConnection) -> anyhow::Result<Self> {

        Ok(Self {
            db_conn: ArcSwap::new(Arc::new(db_conn)),
            settings: ArcSwap::new(Arc::new((*settings).clone())),
        })
    }
}