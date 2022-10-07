// `serde`: basic serialization/deserialization operations
// `serde_derive`: feature tat allows to using struct
use serde_derive::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Config {
    input: Input,
    postgresql: Postgresql,
}

fn main() {
    let config_const_values: Config = {
        let config_path = std::env::args().nth(1).unwrap();
        let config_text = std::fs::read_to_string(&config_path).unwrap();

        toml::from_str(&config_text).unwrap()
    };

    let database = config_const_values.postgresql.database;
    let host = config_const_values.postgresql.host;

    // 5. Get and show one config value.
    println!("Host: {} of the [postgresql].database: {}", host, database);
}
