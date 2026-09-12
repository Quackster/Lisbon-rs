//! Entry point for the web service.

fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();

    println!("Lisbon Web (Rust port)");
    lisbon_web::havana_web::HavanaWeb::main_entry();
}
