use rinf::RustSignal;
use serde::Serialize;

#[derive(Serialize, RustSignal)]
pub struct OnWindowOpenedOutSignal;

#[derive(Serialize, RustSignal)]
pub struct OnWindowClosedOutSignal;

#[derive(Serialize, RustSignal)]
pub struct OnKeyboardStateOutSignal {
    pub using: bool,
}
