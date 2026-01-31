use tinic_database::tinic_database_connection::TinicDbConnection;
use tinic_super::{
    art::helper::ThumbnailEventType, cores::CoreEventType, event::TinicSuperEventListener,
    infos::helper::InfoEventType, rdb_manager::helper::RdbEventType,
};

pub struct TinicSuperEvent {
    pub db_conn: TinicDbConnection,
}

impl TinicSuperEventListener for TinicSuperEvent {
    fn on_thumbnail_evnt(&self, event: ThumbnailEventType) {
        todo!()
    }

    fn on_info_event(&self, event: InfoEventType) {
        todo!()
    }

    fn on_core_event(&self, event: CoreEventType) {
        todo!()
    }

    fn on_rdb_event(&self, state: RdbEventType) {
        todo!()
    }
}
