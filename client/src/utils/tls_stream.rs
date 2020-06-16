use {
    crate::{error::BariumResult, consts::TCP_TIMEOUT},
    std::{net::{TcpStream, ToSocketAddrs}, time::Duration},
    native_tls::{TlsConnector, TlsStream}
};

pub fn new_tls_stream(address: &String, port: u16, allow_invalid_cert: bool) -> BariumResult<TlsStream<TcpStream>> {

    let addr = format!("{}:{}", address, port).to_socket_addrs()?.nth(0).unwrap();

    let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(TCP_TIMEOUT))?;
    let tls_connector = TlsConnector::builder().danger_accept_invalid_certs(allow_invalid_cert).build()?;
    let tls_stream = tls_connector.connect(address.as_str(), stream)?;

    Ok(tls_stream)

}
