use crate::WhoIs;

impl Default for WhoIs {
    fn default() -> Self {
        let servers = include_str!("../data/servers.json");
        match WhoIs::from_string(servers) {
            Ok(whois) => whois,
            Err(err) => panic!("{}", err),
        }
    }
}
