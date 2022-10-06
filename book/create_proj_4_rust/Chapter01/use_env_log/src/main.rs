#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    warn!("Warning mg");
    info!("Information message");
    debug!("Debugging message");
    error!("Error·msg");
}
