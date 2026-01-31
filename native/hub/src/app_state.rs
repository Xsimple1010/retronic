use crate::{ipc::Ipc, tinic_super_event_listener::TinicSuperEvent};
use std::sync::Arc;
use tinic_database::{query, tinic_database_connection::TinicDbConnection};
use tinic_super::{ErrorHandle, RetroPaths, tinic_super::TinicSuper};
use tokio::sync::Mutex;

pub struct AppState {
    pub ipc: Ipc,
    pub tinic_super: Mutex<Option<TinicSuper>>,
    pub base_retro_path: Mutex<Option<RetroPaths>>,
    pub db_conn: Mutex<Option<TinicDbConnection>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ipc: Ipc::new(),
            tinic_super: Mutex::new(None),
            base_retro_path: Mutex::new(None),
            db_conn: Mutex::new(None),
        }
    }

    pub async fn start_app(
        &self,
        tinic_ipc_path: String,
        base_retro_path: String,
    ) -> Result<(), ErrorHandle> {
        self.ipc.start(tinic_ipc_path)?;

        self.base_retro_path
            .lock()
            .await
            .replace(RetroPaths::from_base(base_retro_path.clone())?);

        let conn = TinicDbConnection::new(base_retro_path.clone().into())?;
        query::create_game_table(&conn)?;

        self.db_conn.lock().await.replace(conn.clone());

        self.tinic_super.lock().await.replace(TinicSuper::new(
            RetroPaths::from_base(base_retro_path.clone())?,
            Arc::new(TinicSuperEvent {
                db_conn: conn.clone(),
            }),
        ));

        Ok(())
    }

    pub async fn get_base_retro_path(&self) -> Option<RetroPaths> {
        match &*self.base_retro_path.lock().await {
            Some(base_retro_path) => Some(base_retro_path.clone()),
            None => None,
        }
    }
}
