use secrecy::{Secret, ExposeSecret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("failed to determine the current dfirectory");
    let configuration_directory = base_path.join("configuration");

    // detect the running environment
    // default to 'local' if unspecified
    // parse into an Environment type, since the try_from mandates a proper value
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("failed to parse APP_ENVIRONMENT");

    // the file name is the env + .yaml
    let environment_filename = format!("{}.yaml", environment.as_str());
    
    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::File::from(configuration_directory.join("base.yaml")))
        .add_source(config::File::from(configuration_directory.join(&environment_filename)))	
        .build()?;
    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}


// the possible runtime environment for our application
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
	match self {
	    Environment::Local => "local",
	    Environment::Production => "production",
	}
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
	match value.to_lowercase().as_str() {
	    "local" => Ok(Self::Local),
	    "production" => Ok(Self::Production),
	    other => Err(format!(
		"{} is not supported environment. User either 'local' or 'production'.",
		other
	    )),
	}
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
	    self.password.expose_secret(),
	    self.host,
	    self.port,
	    self.database_name
        ))
    }
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
	    self.password.expose_secret(),
	    self.host,
	    self.port
        ))
    }
}
