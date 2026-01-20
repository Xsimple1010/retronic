use crate::game_info_to_game_info_db::game_info_to_db;
use crate::signals::rust_to_dart::core::OnCoreInstalledOutSignal;
use crate::signals::rust_to_dart::download::{
    OnDownloadCompletedOutSignal, OnDownloadOutSignal, OnExtractFileOutSignal,
};
use rinf::RustSignal;
use tinic_database::model::GameInfoInDb;
use tinic_database::query::insert_game_infos;
use tinic_database::tinic_database_connection::TinicDbConnection;
use tinic_super::DownloadProgress;
use tinic_super::event::TinicSuperEventListener;
use tinic_super::rdb_manager::game_model::GameInfo;

pub struct TinicSuperEvent {
    pub db_conn: TinicDbConnection,
}

impl TinicSuperEventListener for TinicSuperEvent {
    fn downloading(&self, progress: DownloadProgress) {
        match progress {
            DownloadProgress::Started(file_name) => {
                println!("Download started");
            }
            DownloadProgress::Progress(file_name, progress) => {
                OnDownloadOutSignal {
                    file_name,
                    progress,
                }
                .send_signal_to_dart();
            }
            DownloadProgress::Completed(file_name) => {
                OnDownloadCompletedOutSignal { file_name }.send_signal_to_dart();
            }
        }
    }

    fn extract_file(&self, file_name: String) {
        OnExtractFileOutSignal { file_name }.send_signal_to_dart();
    }

    fn rdb_read(&self, _game_info: Vec<GameInfo>) {
        let d: Vec<GameInfoInDb> = _game_info.into_iter().map(game_info_to_db).collect();
        let _ = insert_game_infos(&self.db_conn, &d);
    }

    fn core_installed(&self, core_name: String) {
        OnCoreInstalledOutSignal { core_name }.send_signal_to_dart();
    }
}
