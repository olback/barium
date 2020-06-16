use {
    crate::{new_err, error::BariumResult, utils::new_tls_stream},
    glib::{MainContext, Receiver, Priority},
    barium_shared::{ToServer, ToClient, ServerProperties},
    rmp_serde,
    std::{thread, io::{Read, Write}}
};

pub fn get_server_properties(address: String, port: u16, allow_invalid_cert: bool) -> Receiver<BariumResult<ServerProperties>> {

    let (tx, rx) = MainContext::channel::<BariumResult<ServerProperties>>(Priority::default());

    thread::spawn(move || {

        let get_server_properties_inner = move || -> BariumResult<ServerProperties> {

            let mut tls_stream = new_tls_stream(&address, port, allow_invalid_cert)?;

            let request = ToServer::GetProperties;
            let mut request_bytes = rmp_serde::to_vec(&request)?;
            tls_stream.write_all(&mut request_bytes)?;

            let mut reponse_bytes = [0u8; 64];
            let len = tls_stream.read(&mut reponse_bytes)?;

            let reponse: ToClient = rmp_serde::from_slice(&reponse_bytes[0..len])?;

            match reponse {
                ToClient::Properties(props) => Ok(props),
                _ => Err(new_err!("Invalid response data"))
            }

        };

        tx.send(get_server_properties_inner()).unwrap();

    });

    rx

}

pub fn verify_server_password(address: String, port: u16, allow_invalid_cert: bool, password: String) -> Receiver<BariumResult<bool>> {

    let (tx, rx) = MainContext::channel::<BariumResult<bool>>(Priority::default());

    thread::spawn(move || {

        let verify_server_password_inner = move || -> BariumResult<bool> {

            let mut tls_stream = new_tls_stream(&address, port, allow_invalid_cert)?;

            let request = ToServer::VerifyPassword(password);
            let mut request_bytes = rmp_serde::to_vec(&request)?;
            tls_stream.write_all(&mut request_bytes)?;

            let mut reponse_bytes = [0u8; 256];
            let len = tls_stream.read(&mut reponse_bytes)?;

            let reponse: ToClient = rmp_serde::from_slice(&reponse_bytes[0..len])?;

            match reponse {
                ToClient::PasswordOk(valid) => Ok(valid),
                _ => Err(new_err!("Invalid response data"))
            }

        };

        tx.send(verify_server_password_inner()).unwrap();

    });

    rx

}
