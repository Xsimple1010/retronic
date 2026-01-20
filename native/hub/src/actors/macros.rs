#[macro_export]
macro_rules! impl_dart_actor {
    (
        actor = $actor:ty,
        signal = $signal:ty
    ) => {
        impl $crate::actors::traits::RetronicDartActor<$signal> for $actor {
            fn build(
                app_state: std::sync::Arc<AppState>,
                owned_tasks: tokio::task::JoinSet<()>,
            ) -> Self {
                Self {
                    owned_tasks,
                    app_state,
                }
            }
        }

        impl $crate::actors::traits::DartActorNotify<$signal> for $actor {}

        impl messages::actor::Actor for $actor {}
    };
}
