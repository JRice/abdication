//! ABDrum as a standalone app (for trying the editor without a host).
//! `cargo run --release --features standalone --bin abdrum-standalone -- --help`
fn main() {
    nih_plug::nih_export_standalone::<abdrum::ABDrum>();
}
