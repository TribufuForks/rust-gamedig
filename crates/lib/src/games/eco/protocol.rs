use crate::eco::{Response, Root};
use crate::http::{HTTPSettings, HttpClient};
use crate::{GDResult, TimeoutSettings};
use std::net::{IpAddr, SocketAddr};

/// Query a eco server.
#[inline]
pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, &None) }

/// Query a eco server.
pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
) -> GDResult<Response> {
    let address = &SocketAddr::new(*address, port.unwrap_or(3001));
    let mut client = HttpClient::new(
        address,
        timeout_settings,
        HTTPSettings {
            protocol: crate::http::Protocol::HTTP,
            hostname: None,
        },
    )?;

    let response = client.get_json::<Root>("/frontpage")?;

    Ok(response.into())
}
