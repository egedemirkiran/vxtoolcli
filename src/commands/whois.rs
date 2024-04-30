use crate::Error;
use dns_lookup::lookup_host;
use serde::{Deserialize, Serialize};

#[derive(clap::Args)]
pub struct Args {
    #[clap(long)]
    pub host: Option<String>,
    #[clap(long)]
    pub ip: Option<String>,
}

pub fn get_addr_info_by_host(hostname: &str) -> Result<String, ()> {
    match lookup_host(hostname) {
        Ok(addrs) => Ok(addrs[0].to_string()),
        Err(_) => Err(()),
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct IPWhoIs {
    ip: String,
    success: bool,
    #[serde(rename = "type")]
    type_: String,
    continent: String,
    country: String,
    region: String,
    city: String,
    #[serde(rename = "latitude")]
    lat: f64,
    #[serde(rename = "longitude")]
    lon: f64,
    timezone: String,
    asn: String,
    org: String,
    isp: String,
}

pub fn whois_by_ip(ip: &str) -> Result<String, Error> {
    let body: Result<Option<IPWhoIs>, crate::error::Error> = crate::send_request(
        format!("https://ipwhois.app/json/{}", ip).as_str(),
        reqwest::Method::GET,
        None,
        None,
    );

    match body {
        Ok(Some(data)) => Ok(format!(
            "IP: {}\nContinent: {}\nCountry: {}\nRegion: {}\nCity: {}\nLatitude: {}\nLongitude: {}\nTimezone: {}\nASN: {}\nOrganization: {}\nISP: {}",
            data.ip, data.continent, data.country, data.region, data.city, data.lat, data.lon, data.timezone, data.asn, data.org, data.isp
        )),
        Err(error) => Err(error),
        _ => Err(Error::Unknown),
    }
}
