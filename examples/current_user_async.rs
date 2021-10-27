use env_logger::Env;
use traduora::api::users::Me;
use traduora::AsyncQuery;
use traduora::AsyncTraduora;
use traduora::Login;
use traduora::TraduoraError;

#[tokio::main]
async fn main() -> Result<(), TraduoraError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    let login = Login::password("test@test.test", "12345678");
    let client = AsyncTraduora::with_auth_insecure("localhost:8080", login).await?;
    let result = Me.query_async(&client).await?;
    log::info!("{:?}", result);
    Ok(())
}
