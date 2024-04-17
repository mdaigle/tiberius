use crate::Config;
use futures_util::io::{AsyncRead, AsyncWrite};

mod native_tls_stream;

pub(crate) use native_tls_stream::TlsStream;

pub(crate) async fn create_tls_stream<S: AsyncRead + AsyncWrite + Unpin + Send>(
    config: &Config,
    stream: S,
) -> crate::Result<TlsStream<S>> {
    native_tls_stream::create_tls_stream(config, stream).await
}
