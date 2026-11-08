//! Amara as a standalone app, for trying the panel without a host.
//! `cargo run --release --features standalone --bin amara-standalone -- --help`
fn main() {
    nih_plug::nih_export_standalone::<amara::Amara>();
}
