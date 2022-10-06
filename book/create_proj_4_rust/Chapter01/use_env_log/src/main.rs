#[macro_use]
extern crate log;
// use cmd  RUST_LOG=debug ./target/debug/use_env_log
// define temp env variable RUST_LOG, debug is the value

fn main() {
    env_logger::init();

    warn!("Warning mg");
    info!("Information message");
    debug!("Debugging message");
    error!("Error·msg");
}
