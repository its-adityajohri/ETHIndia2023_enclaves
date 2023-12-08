use bindings::outbox::SentMessageFilter;
use ethers::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{str::FromStr, sync::Arc};

lazy_static! {
    static ref CHAIN_HASHMAP: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "ethereum wss url");
        m.insert(2, "ropsten wss url");
        m
    };
}

lazy_static! {
    static ref INBOX_CONTRACT: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "ethereum inbox address");
        m.insert(2, "ropsten inbox address");
        m
    };
}

lazy_static! {
    static ref OUTBOX_CONTRACT: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "ethereum OUTBOX address");
        m.insert(2, "ropsten OUTBOX address");
        m
    };
}

pub async fn parse(from_chain_id: u64, to_chain_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let from_wss = CHAIN_HASHMAP
        .get(&from_chain_id)
        .ok_or_else(|| "From chain ID not found in the hashmap")?;
    let to_wss = CHAIN_HASHMAP
        .get(&to_chain_id)
        .ok_or_else(|| "To chain ID not found in the hashmap")?;

    let inbox_contract = INBOX_CONTRACT
        .get(&from_chain_id)
        .ok_or_else(|| "From chain ID not found in the hashmap")?;
    let outbox_contract = OUTBOX_CONTRACT
        .get(&from_chain_id)
        .ok_or_else(|| "From chain ID not found in the hashmap")?;

    // Example operation: print the RPC URLs
    println!("From WSS URL: {}", from_wss);
    println!("To WSS URL: {}", to_wss);

    let key = "0xca84b7b947ced9b5476ea2ed1605e3bb28f867e79807ae134926fa6242cfaf2d"; //enclave does not support env directly, hence adding it directly for now

    let from_signer = key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(from_chain_id);
    let to_signer = key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(to_chain_id);

    let from_provider_ws = Provider::<Ws>::connect_with_reconnects(&from_wss, 2)
        .await?
        .with_signer(from_signer);

    let to_provider_ws = Provider::<Ws>::connect_with_reconnects(&to_wss, 2)
        .await?
        .with_signer(to_signer);

    let from_client = Arc::new(from_provider_ws);

    let inbox_contract_address = Address::from_str(inbox_contract)?;

    let event = Contract::event_of_type::<SentMessageFilter>(Arc::clone(&from_client))
        .address(ValueOrArray::Array(vec![inbox_contract_address]));

    let mut stream = event.subscribe_with_meta().await?;

    while let Some(Ok((event, meta))) = stream.next().await {}
    Ok(())
}
