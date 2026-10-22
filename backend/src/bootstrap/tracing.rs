use std::fs::File;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "habit_tracking_backend=debug,tower_http=debug".into());

    // Console (pretty logs).
    let console_layer = fmt::layer().with_target(false).with_level(true).pretty();

    // File (structured JSON logs).
    let file = File::create("app.log").expect("cannot create log file");
    let json_layer = fmt::layer()
        .json()
        .with_writer(file)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(json_layer)
        .try_init()
        .ok();
}
