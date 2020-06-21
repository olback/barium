use {
    std::{str::FromStr, cmp::Ordering},
    trust_dns_client::{
        op::DnsResponse,
        client::{Client, SyncClient},
        udp::UdpClientConnection,
        rr::{DNSClass, Name, RData, Record, RecordType},
        proto::rr::rdata::srv::SRV
    },
    crate::error::BariumResult
};

pub fn get_srv_addr(addr: &String) -> BariumResult<Option<String>> {

    let conn = UdpClientConnection::new("1.0.0.1:53".parse()?)?;
    let client = SyncClient::new(conn);
    let name = Name::from_str(&format!("_barium._tls.{}", addr))?;
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::SRV)?;
    let answers: &[Record] = response.answers();

    let host = match answers.len() {

        0 => None,

        1 => match answers[0].rdata() {

            &RData::SRV(ref srv) => Some(srv.target().to_ascii()),
            _ => None

        },

        _ => {

            let mut answers = answers.iter()
                .filter_map(|ans| match ans.rdata() {
                    &RData::SRV(ref srv) => Some(srv),
                    _ => None
                })
                .collect::<Vec<&SRV>>();

            answers.sort_by(|a, b| {
                match b.priority() > a.priority() {
                    true => Ordering::Greater,
                    false => Ordering::Less
                }
            });

            Some(answers[0].target().to_ascii())

        }

    };

    Ok(match host {

        Some(h) => match h.ends_with(".") {
            true => Some(h.as_str()[..h.len() - 1].to_string()),
            false => Some(h)
        },

        None => None

    })

}
