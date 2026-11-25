
/// Theo dõi thư mục migrations để Cargo biên dịch lại khi migration thay đổi.
/// Watch the migrations directory so Cargo rebuilds when migration files change.
fn main() {
    println!("cargo:rerun-if-changed=migrations");

    if let Ok(entries) = std::fs::read_dir("migrations") {
        for entry in entries.flatten() {
            println!(
                "cargo:rerun-if-changed={}",
                entry.path().display()
            );
        }
    }
}
