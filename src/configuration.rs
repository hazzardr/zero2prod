
#[derive(serde::Deserialize)]
pub struct Settings {
    pub http_port: u16,
    pub database: Database
}

#[derive(serde::Deserialize)]
pub struct Database {
    pub user: String,
    pub pass: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.pass, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<Settings>()

}