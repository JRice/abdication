//! Raycaster as a standalone app, for trying the panel without a host.
//! `cargo run --release --features standalone --bin raycaster-standalone -- --help`
fn main() {
    nih_plug::nih_export_standalone::<raycaster::Raycaster>();
}
