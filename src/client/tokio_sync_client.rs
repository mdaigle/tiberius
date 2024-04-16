use tokio::{net::TcpStream, runtime::{Builder, Runtime}};
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::{
    tds::stream::QueryStream, Client, Config, ToSql
};


pub struct TokioSyncClient {
    runtime: Runtime,
    client: Client<Compat<TcpStream>>,
}

impl TokioSyncClient {

    pub fn new(config: Config) -> Self {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let client = runtime.block_on(async {
            let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
            tcp.set_nodelay(true);
            Client::connect(config, tcp.compat_write()).await.unwrap()
        });

        Self { 
            runtime,
            client 
        }
    }

    pub fn query<'a, 'b>(&'a mut self, sql: &'b str, params: &[&'b dyn ToSql]) -> crate::Result<QueryStream<'a>> 
    where
    'a: 'b,
    {
        self.runtime.block_on(async {
            self.client.query(sql, params).await
        })
    }
}

