use bindings::outbox::SentMessageFilter;
use ethers::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{str::FromStr, sync::Arc};

lazy_static! {
    static ref CHAIN_HASHMAP: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(44787, "wss://alfajores-forno.celo-testnet.org/ws");
        m.insert(
            11155111,
            "wss://eth-sepolia.g.alchemy.com/v2/RMhzE5IvsGlGyO3kuREyJEugrSOeHsB2",
        );
        m
    };
}

lazy_static! {
    static ref INBOX_CONTRACT: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(44787, "0x42ACd5984Ef828154E478da4Ca1e6f1dd2b7ebd0");
        m.insert(11155111, "0x42ACd5984Ef828154E478da4Ca1e6f1dd2b7ebd0");
        m
    };
}

lazy_static! {
    static ref OUTBOX_CONTRACT: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(44787, "0x4C3Eae65dCAdA64979691a72cfB42cF67bd9BD3C");
        m.insert(11155111, "0x4C3Eae65dCAdA64979691a72cfB42cF67bd9BD3C");
        m
    };
}

lazy_static! {
    static ref TOKEN_BRIDGE: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(44787, "0x4C3Eae65dCAdA64979691a72cfB42cF67bd9BD3C");
        m.insert(11155111, "0x4C3Eae65dCAdA64979691a72cfB42cF67bd9BD3C");
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
        .ok_or_else(|| "To chain ID not found in the hashmap")?;

    // Example operation: print the RPC URLs

    println!("From WSS URL: {}", from_wss);
    println!("To WSS URL: {}", to_wss);

    //enclave does not support env directly, hence adding it directly for now
    let key = "0xca84b7b947ced9b5476ea2ed1605e3bb28f867e79807ae134926fa6242cfaf2d";

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
    let to_client = Arc::new(to_provider_ws);

    let outbox_contract_address = Address::from_str(outbox_contract)?;

    let event = Contract::event_of_type::<SentMessageFilter>(Arc::clone(&from_client))
        .address(ValueOrArray::Array(vec![outbox_contract_address]));

    let mut stream = event.subscribe_with_meta().await?;

    let inbox_contract_address = Address::from_str(inbox_contract)?;
    let inbox = Arc::new(bindings::inbox::Inbox::new(
        inbox_contract_address,
        Arc::clone(&to_client),
    ));

    let to_chain_token_bridge = TOKEN_BRIDGE
        .get(&to_chain_id)
        .ok_or_else(|| "to chain ID not found in the hashmap")?;

    let to_chain_token_bridge = Address::from_str(to_chain_token_bridge)?;
    let to_token_bridge = Arc::new(bindings::token_bridge::TokenBridge::new(
        to_chain_token_bridge,
        Arc::clone(&to_client),
    ));

    while let Some(Ok((event, _))) = stream.next().await {
        let source_chain_id = event.from_chain_id;
        let source_message_id = event.message_id;
        let from = event.from;
        let to = event.to;
        let data = event.data;

        inbox
            .receive_message(source_message_id, source_chain_id, from, to, data)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        to_token_bridge
            .release_token(source_chain_id, source_message_id)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
    }

    Ok(())
}
