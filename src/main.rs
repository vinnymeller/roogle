use roogle::cli;
fn main() {
    if let Ok(log_level) = std::env::var("ROOGLE_LOG") {
        env_logger::Builder::new().parse_filters(&log_level).init();
    } else {
        env_logger::Builder::new().parse_filters("off").init();
    }

    cli::parse();
}
