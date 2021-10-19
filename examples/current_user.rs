use simple_logger::SimpleLogger;
use traduora::api::users::Me;
use traduora::Login;
use traduora::Query;
use traduora::Traduora;
use traduora::TraduoraError;

fn main() -> Result<(), TraduoraError> {
    SimpleLogger::new().init().unwrap();
    let login = Login::password("test@test.test", "12345678");
    let client = Traduora::with_auth_insecure("localhost:8080", login)?;
    let result = Me.query(&client)?;
    log::info!("{:?}", result);
    Ok(())
}
