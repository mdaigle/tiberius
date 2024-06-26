use once_cell::sync::Lazy;
use std::env;
use tiberius::{Client, Config};
use tokio::{net::TcpStream, runtime::Runtime};
use tokio_util::compat::TokioAsyncWriteCompatExt;

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:localhost,51210;IntegratedSecurity=true;TrustServerCertificate=true;database=tiberius".to_owned()
    })
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&CONN_STR)?;

        let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
        tcp.set_nodelay(true);

        let mut client = Client::connect(config, tcp.compat_write()).await.unwrap();

        let stream = client.query("SELECT * from people", &[]).await.unwrap();
        let row = stream.into_row().await.unwrap().unwrap();

        println!("{:?}", row);
        assert_eq!(Some("Malcolm"), row.get(0));

    Ok(())
}