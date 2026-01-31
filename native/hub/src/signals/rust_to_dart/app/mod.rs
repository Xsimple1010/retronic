use rinf::RustSignal;
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct OnAppExitOutSignal;

#[derive(Serialize, RustSignal)]
pub struct OnAppStartOutSignal;
