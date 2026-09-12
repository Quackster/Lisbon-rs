//! Entry point — mirrors `net.h4bbo.lisbon.Lisbon.main`.
//!
//! Mirrors `Lisbon.main`: check the logging configuration, print the banner,
//! then run the full `Lisbon` bootstrap (`Lisbon::run`), which loads config,
//! connects storage, bootstraps the managers, and starts the Netty / MUS /
//! RCON servers on a tokio runtime.

fn main() {
    let _ = lisbon_server::util::config::logging_configuration::LoggingConfiguration::check_logging_config();

    println!("  _      _     _                 ");
    println!(" | |    (_)   | |                ");
    println!(" | |     _ ___| |__   ___  _ __  ");
    println!(" | |    | / __| '_ \\ / _ \\| '_ \\ ");
    println!(" | |____| \\__ \\ |_) | (_) | | | |");
    println!(" |______|_|___/_.__/ \\___/|_| |_|");
    println!("");
    tracing::info!("Version: {}", lisbon_server::SERVER_VERSION);

    lisbon_server::lisbon::Lisbon::run();
}
