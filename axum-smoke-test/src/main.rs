use axum::Router;
use std::net::SocketAddr;
use std::time::Duration;

use anvil_core::startup::Startup;
use anvil_core::health::DegradationReason;
use anvil_core::lifecycle::LifecycleState;

use tracing_subscriber::fmt::Subscriber;

#[tokio::main]
async fn main() {
    // observability ditentukan user (bukan core / adapter)
    Subscriber::builder()
        .with_env_filter("info")
        .init();

    // startup orchestrator (core)
    let startup = Startup::new();

    // simulasi: service belum siap karena DB
    let db_down = DegradationReason::new(
        "db_unavailable",
        "database connection not established",
    );
    startup.health().add_degradation(db_down);

    // router dari adapter
    let app: Router =
        anvil_adapter_axum::health_routes(startup.health());

    // simulasi service siap setelah 3 detik
    tokio::spawn({
        let startup = startup;
        async move {
            // simulasi startup phase
            startup
                .lifecycle()
                .transition(LifecycleState::Starting)
                .unwrap();

            tokio::time::sleep(Duration::from_secs(3)).await;

            startup.mark_ready().unwrap();
            tracing::info!("service marked READY");
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}