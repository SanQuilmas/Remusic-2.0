// Prod Env
// rdkafka = { version = "0.37.0"}
pub const DATABASE_URL: &str = "postgresql://user:password@postgresdb:5432/postgres";
pub const STATIC_FOLDER: &str = "/app/remusic/static/";
pub const IP_BIND: &str = "0.0.0.0";
pub const MACHINE_FOLDER_PATH: &str = "/app/remusic/static/";
pub const MACHINE_FOLDER_URL: &str = "http://backend:8080/static/";
pub const KAFKA_BROKER: &str = "broker:9092";

//Local Dev Env
// rdkafka = { version = "0.37.0", features = ["dynamic-linking"] }
// pub const DATABASE_URL: &str = "postgresql://user:password@localhost:6432/postgres";
// pub const STATIC_FOLDER: &str = "/home/leonel/Virtual Machines/remusic/static";
// pub const IP_BIND: &str = "127.0.0.1";
// pub const MACHINE_FOLDER_PATH: &str = "/home/leonel/Virtual Machines/remusic/static/";
// pub const MACHINE_FOLDER_URL: &str = "http://localhost:8080/static/";
// pub const KAFKA_BROKER: &str = "localhost:9092";

//Kafka topics used
pub const KAFKA_NOT_UPDATED_TOPIC: &str = "newly_created";
pub const KAFKA_ALREADY_UPDATED_TOPIC: &str = "newly_updated";
