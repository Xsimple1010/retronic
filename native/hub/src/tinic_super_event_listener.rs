use tinic_super::event::TinicSuperEventListener;

struct TinicSuperEvent;

impl TinicSuperEventListener for TinicSuperEvent {
    fn downloading(&self, file_name: String, percent: f32) {
        todo!()
    }

    fn extract_file(&self, file_name: String) {
        todo!()
    }

    fn download_completed(&self, file_name: String) {
        todo!()
    }
}