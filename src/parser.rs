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
        m.insert(44787, "0xAb92552d917E68418345e1C11Eb903aC0d9c72B9");
        m.insert(11155111, "0xad88364BFbE1048cd09628a457403C9617b33fCf");
        m
    };
}

lazy_static! {
    static ref OUTBOX_CONTRACT: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(44787, "0x56Dd701e2e1bb37515ef6F30Cd5272b7dD9779C2");
        m.insert(11155111, "0xdfd15A995548c9dbFff66ce02351dB74a7B1b8D0");
        m
    };
}

lazy_static! {
    static ref TOKEN_BRIDGE: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(44787, "0xdef67A47c508cdc052f352389732E05Ee00AAB54");
        m.insert(11155111, "0x823012d8cf5abAF798b731Aef12576d9DE0E5cEb");
        m
    };
}

pub async fn parse(from_chain_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let from_wss = CHAIN_HASHMAP
        .get(&from_chain_id)
        .ok_or_else(|| "From chain ID not found in the hashmap")?;

    let outbox_contract = OUTBOX_CONTRACT
        .get(&from_chain_id)
        .ok_or_else(|| "From chain ID not found in the hashmap")?;

    // Example operation: print the RPC URLs

    println!("From WSS URL: {}", from_wss);

    //enclave does not support env directly, hence adding it directly for now
    let key = "0xca65f4b51f9bce13a20e8de77c7108613b6151ed4b466a38969f1575fb361033";

    let from_signer = key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(from_chain_id);

    let from_provider_ws = Provider::<Ws>::connect_with_reconnects(&from_wss, 2)
        .await?
        .with_signer(from_signer);

    let from_client = Arc::new(from_provider_ws);

    let outbox_contract_address = Address::from_str(outbox_contract)?;

    let event = Contract::event_of_type::<SentMessageFilter>(Arc::clone(&from_client))
        .address(ValueOrArray::Array(vec![outbox_contract_address]));

    let mut stream = event.subscribe_with_meta().await?;

    while let Some(Ok((event, _))) = stream.next().await {
        let source_chain_id = event.from_chain_id;
        let source_message_id = event.message_id;
        let destination_chain_id = event.to_chain_id;
        let from = event.from;
        let to = event.to;
        let data = event.data;

        println!(
            "Received event from chainid {} with message id {}, to chain id {}",
            source_chain_id, source_message_id, destination_chain_id
        );

        let to_wss = CHAIN_HASHMAP
            .get(&destination_chain_id.as_u64())
            .ok_or_else(|| "From chain ID not found in the hashmap")?;

        let inbox_contract = INBOX_CONTRACT
            .get(&destination_chain_id.as_u64())
            .ok_or_else(|| "To chain ID not found in the hashmap")?;

        let to_signer = key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(destination_chain_id.as_u64());

        let to_provider_ws = Provider::<Ws>::connect_with_reconnects(&to_wss, 2)
            .await?
            .with_signer(to_signer.clone());

        let to_client = Arc::new(to_provider_ws);

        let inbox_contract_address = Address::from_str(inbox_contract)?;

        let inbox = Arc::new(bindings::inbox::Inbox::new(
            inbox_contract_address,
            Arc::clone(&to_client),
        ));

        // dbg!(to_signer.address());
        // let balance_of_signer = to_client
        //     .get_balance(to_signer.address(), None)
        //     .await
        //     .unwrap();

        // dbg!(balance_of_signer);

        let tx = inbox
            .receive_message(source_message_id, source_chain_id, from, to, data)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        println!(
            "Inbox Transaction: {}",
            h256_to_hex_string(tx.unwrap().transaction_hash)
        );

        let token_bridge_contract = TOKEN_BRIDGE
            .get(&destination_chain_id.as_u64())
            .ok_or_else(|| "To chain ID not found in the hashmap")?;

        let token_bridge_contract = Address::from_str(token_bridge_contract)?;

        let to_token_bridge =
            bindings::token_bridge::TokenBridge::new(token_bridge_contract, Arc::clone(&to_client));
        let tx = to_token_bridge
            .release_token(source_chain_id, source_message_id)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        println!(
            "Bridge Transaction: {}",
            h256_to_hex_string(tx.unwrap().transaction_hash)
        );
    }

    Ok(())
}

fn h256_to_hex_string(hash: H256) -> String {
    hash.as_bytes()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}
