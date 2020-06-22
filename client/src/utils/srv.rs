use {
    std::{str::FromStr, cmp::Ordering, sync::Mutex, collections::HashMap},
    trust_dns_client::{
        op::DnsResponse,
        client::{Client, SyncClient},
        udp::UdpClientConnection,
        rr::{DNSClass, Name, RData, Record, RecordType},
        proto::rr::rdata::srv::SRV
    },
    crate::error::BariumResult,
    lazy_static::lazy_static,
    padlock,
    log::debug,
    glib::clone
};

lazy_static! {
    static ref SRV_CACHE: Mutex<HashMap<String, Option<String>>> = Mutex::new(HashMap::<String, Option<String>>::new());
}

pub fn get_srv_addr(addr: &String) -> BariumResult<Option<String>> {

    let cache = padlock::mutex_lock(&*SRV_CACHE, clone!(@strong addr => move |cache| {
        cache.get(&addr).map(|a| a.clone())
    }));

    if let Some(cached_addr) = cache {
        debug!("Got SRV cache hit {:?} for {}", cached_addr, addr);
        return Ok(cached_addr)
    }

    debug!("{} not found in SRV cache", addr);

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

    let trimmed_host = match host {

        Some(h) => match h.ends_with(".") {
            true => Some(h.as_str()[..h.len() - 1].to_string()),
            false => Some(h)
        },

        None => None

    };

    padlock::mutex_lock(&*SRV_CACHE, clone!(@strong addr, @strong trimmed_host => move |cache| {
        debug!("Saving {} => {:?} in SRV cache", addr, trimmed_host);
        cache.insert(addr, trimmed_host);
    }));

    Ok(trimmed_host)

}
