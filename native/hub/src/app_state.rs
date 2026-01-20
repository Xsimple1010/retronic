use crate::{ipc::Ipc, tinic_super_event_listener::TinicSuperEvent};
use std::sync::Arc;
use tinic_database::tinic_database_connection::TinicDbConnection;
use tinic_super::{tinic_super::TinicSuper, ErrorHandle, RetroPaths};
use tokio::sync::Mutex;

pub struct AppState {
    pub ipc: Ipc,
    pub tinic_super: Mutex<Option<TinicSuper>>,
    pub base_retro_path: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ipc: Ipc::new(),
            tinic_super: Mutex::new(None),
            base_retro_path: Mutex::new(None),
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
            .replace(base_retro_path.clone());

        self.tinic_super.lock().await.replace(TinicSuper::new(
            RetroPaths::from_base(base_retro_path.clone())?,
            Arc::new(TinicSuperEvent {
                db_conn: TinicDbConnection::new(base_retro_path.into())?,
            }),
        ));

        Ok(())
    }

    pub async fn get_base_retro_path(&self) -> Option<String> {
        match &*self.base_retro_path.lock().await {
            Some(base_retro_path) => Some(base_retro_path.clone()),
            None => None,
        }
    }
}
