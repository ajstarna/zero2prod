use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    println!("hello!"); // remove this: just testing cargo chef
    
    // `init` does call `set_logger`, so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let configuration = get_configuration().expect("failed to read configuration.");

    // no longer async, given that we don't actually try to connect!
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");
    
    let address = format!(
	"{}:{}",
	configuration.application.host,	
	configuration.application.port,
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
