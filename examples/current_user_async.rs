use simple_logger::SimpleLogger;
use traduora::api::users::me::Me;
use traduora::api::users::me::UserInfoResponse;
use traduora::api::AsyncQuery;
use traduora::auth::Login;
use traduora::AsyncTraduora;
use traduora::TraduoraError;

#[tokio::main]
async fn main() -> Result<(), TraduoraError> {
    SimpleLogger::new().init().unwrap();
    let login = Login::password("test@test.test", "12345678");
    let client = AsyncTraduora::with_auth_insecure("localhost:8080", login).await?;
    let result: UserInfoResponse = Me.query_async(&client).await?;
    log::info!("{:?}", result);
    Ok(())
}
