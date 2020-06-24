use {
    crate::{error::BariumResult, consts::TCP_TIMEOUT},
    super::get_srv_addr,
    std::{net::{TcpStream, ToSocketAddrs}, time::Duration},
    native_tls::{TlsConnector, TlsStream, Protocol}
};

pub fn new_tls_stream(address: String, port: u16, allow_invalid_cert: bool) -> BariumResult<TlsStream<TcpStream>> {

    let host = get_srv_addr(&address)?.unwrap_or(address);

    let addr = format!("{}:{}", host, port).to_socket_addrs()?.nth(0).unwrap();

    let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(TCP_TIMEOUT))?;
    stream.set_read_timeout(Some(Duration::from_secs(TCP_TIMEOUT)))?;
    stream.set_write_timeout(Some(Duration::from_secs(TCP_TIMEOUT)))?;
    let tls_connector = TlsConnector::builder()
        .min_protocol_version(Some(Protocol::Tlsv12)) // TODO: Tlsv13
        .danger_accept_invalid_certs(allow_invalid_cert)
        .build()?;
    let tls_stream = tls_connector.connect(host.as_str(), stream)?;

    Ok(tls_stream)

}
