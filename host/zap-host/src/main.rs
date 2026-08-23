use std::{process::ExitCode, sync::Arc};

use tokio::net::TcpListener;
use tokio::{signal, sync::oneshot};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use zap_host::{build_router, AppConfig, AppState, LifecycleState};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zap_host=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!(error = %error, "zap-host stopped with an error");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = AppConfig::from_env()?;
    let address = config.bind_addr;
    let shutdown_timeout = config.shutdown_timeout;
    let state = AppState::from_env(config)?;
    let lifecycle = state.lifecycle.clone();
    let router = build_router(state);
    let listener = TcpListener::bind(address).await?;

    tracing::info!(%address, ?shutdown_timeout, "zap-host listening");
    let (drain_started_tx, drain_started_rx) = oneshot::channel();
    let drain_timeout = async move {
        if drain_started_rx.await.is_ok() {
            tokio::time::sleep(shutdown_timeout).await;
            true
        } else {
            false
        }
    };
    tokio::select! {
        result = axum::serve(listener, router).with_graceful_shutdown(shutdown_signal(lifecycle.clone(), drain_started_tx)) => {
            result?;
        }
        timed_out = drain_timeout => {
            if timed_out && lifecycle.is_draining() {
                tracing::warn!(?shutdown_timeout, "zap-host forced shutdown after drain timeout");
            }
        }
    }
    tracing::info!("zap-host shutdown complete");
    Ok(())
}

async fn shutdown_signal(lifecycle: Arc<LifecycleState>, drain_started: oneshot::Sender<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    lifecycle.begin_draining();
    let _ = drain_started.send(());
    tracing::info!("zap-host entering graceful drain");
}
