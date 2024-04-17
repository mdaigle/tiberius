use once_cell::sync::Lazy;
use std::env;
use tiberius::{Config, TokioSyncClient};

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:localhost,51210;IntegratedSecurity=true;TrustServerCertificate=true;database=tiberius".to_owned()
    })
});

fn main() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&CONN_STR)?;

    let mut client = TokioSyncClient::new(config);

    let stream = client.query("SELECT * from people", &[]).unwrap();
    let row = stream.into_row().unwrap().unwrap();

    println!("{:?}", row);
    assert_eq!(Some("Malcolm"), row.get(0));

    Ok(())
}