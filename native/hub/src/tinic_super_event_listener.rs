use rinf::RustSignal;
use tinic_database::{model::GameInfoInDb, query, tinic_database_connection::TinicDbConnection};
use tinic_super::{
    DownloadProgress, ExtractProgress, art::helper::ThumbnailEventType, cores::CoreEventType,
    event::TinicSuperEventListener, infos::helper::InfoEventType,
    rdb_manager::helper::RdbEventType,
};

use crate::{
    game_info_to_game_info_db::game_info_to_db,
    signals::rust_to_dart::{
        database::OnReadRdbProgressOutSignal,
        download::{
            OnDownloadCompletedOutSignal, OnDownloadProgressOutSignal, OnDownloadStartedOutSignal,
        },
        extraction_signal::{OnExtractFinishedOutSignal, OnExtractingOutSignal},
    },
};

pub struct TinicSuperEvent {
    pub db_conn: TinicDbConnection,
}

fn download_event_handle(download_event: DownloadProgress) {
    match download_event {
        DownloadProgress::Started { name } => {
            OnDownloadStartedOutSignal { file_name: name }.send_signal_to_dart();
        }
        DownloadProgress::Progress { name, progress } => {
            OnDownloadProgressOutSignal {
                file_name: name,
                progress,
            }
            .send_signal_to_dart();
        }
        DownloadProgress::Completed { name } => {
            OnDownloadCompletedOutSignal { file_name: name }.send_signal_to_dart();
        }
    }
}

fn extraction_event_handle(extraction_event: ExtractProgress) {
    match extraction_event {
        ExtractProgress::Extracting {
            origin_file,
            inner_file_name,
        } => {
            OnExtractingOutSignal {
                inner_file_name,
                origin_file,
            }
            .send_signal_to_dart();
        }

        ExtractProgress::Finished { origin_file } => {
            OnExtractFinishedOutSignal { origin_file }.send_signal_to_dart();
        }
    }
}

impl TinicSuperEventListener for TinicSuperEvent {
    fn on_thumbnail_evnt(&self, event: ThumbnailEventType) {
        match event {
            ThumbnailEventType::Downloading(download_event) => {
                download_event_handle(download_event)
            }
        }
    }

    fn on_info_event(&self, event: InfoEventType) {
        match event {
            InfoEventType::Downloading(download_event) => download_event_handle(download_event),
            InfoEventType::Extraction(extact_event) => extraction_event_handle(extact_event),
        }
    }

    fn on_core_event(&self, event: CoreEventType) {
        match event {
            CoreEventType::Downloading(download_event) => download_event_handle(download_event),
            CoreEventType::Extraction(extact_event) => extraction_event_handle(extact_event),
        }
    }

    fn on_rdb_event(&self, event: RdbEventType) {
        match event {
            RdbEventType::Downloading(download_event) => download_event_handle(download_event),
            RdbEventType::Extracting(extact_event) => extraction_event_handle(extact_event),
            RdbEventType::ReadProgress {
                total,
                remaining,
                rdb_name,
            } => {
                OnReadRdbProgressOutSignal {
                    total: total as u16,
                    remaining: remaining as u16,
                    console_name: rdb_name,
                }
                .send_signal_to_dart();
            }
            RdbEventType::Reading { game_infos } => {
                let conn = &self.db_conn;
                let game_infos = game_infos
                    .into_iter()
                    .map(|g| game_info_to_db(g))
                    .collect::<Vec<GameInfoInDb>>();

                let _ = query::insert_game_infos(conn, &game_infos);
            }
        }
    }
}
