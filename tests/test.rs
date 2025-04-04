use whois_rs::*;

#[test]
fn test() {
    let who = WhoIs::from_path("data/servers.json").unwrap();

    let result = who.lookup(WhoIsLookupOptions::from_string("magiclen.org").unwrap()).unwrap();
    println!("{}", result);

    let result = who.lookup(WhoIsLookupOptions::from_string("172.105.210.153").unwrap()).unwrap();
    println!("{}", result);
}

#[test]
fn test_default() {
    let who = WhoIs::default();

    let result = who.lookup(WhoIsLookupOptions::from_string("magiclen.org").unwrap()).unwrap();
    println!("{}", result);
    let found = result.find("Domain Name: magiclen.org");
    assert!(found.is_some());
}

#[test]
fn test_srv() {
    let mut who = WhoIs::from_host("whois.arin.net").unwrap();

    assert!(who.can_find_server_for_tld(".lotteryusa.us", "8.8.8.8:53"));

    let result = who.lookup(WhoIsLookupOptions::from_string("lotteryusa.us").unwrap()).unwrap();
    println!("{}", result);
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn test_async() {
    let who = WhoIs::from_path_async("data/servers.json").await.unwrap();

    let result =
        who.lookup_async(WhoIsLookupOptions::from_string("magiclen.org").unwrap()).await.unwrap();
    println!("{}", result);

    let result = who
        .lookup_async(WhoIsLookupOptions::from_string("172.105.210.153").unwrap())
        .await
        .unwrap();
    println!("{}", result);
}
