/// Điểm bắt đầu của chương trình.
/// Application entry point.
#[tokio::main]
async fn main() {
    if let Err(error) = forgerust::bootstrap::startup::run().await {
        eprintln!("ForgeRust startup failed: {error}");
        std::process::exit(1);
    }
}
