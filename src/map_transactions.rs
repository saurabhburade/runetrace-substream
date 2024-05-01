use bitcoin::{ Address, Network, Script };

use crate::pb::btc::transaction_meta::v1::{ Block, Transactions, Transaction, Vout };

#[substreams::handlers::map]
fn map_transactions(blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let transactions: Vec<Transaction> = blk.tx
        .iter()
        .enumerate()
        .map(|(_index, trans)| {
            let vouts_parsed = trans.vout
                .iter()
                .map(|_vout| Vout {
                    value: _vout.value.clone(),
                    n: _vout.n.clone(),
                    script_pub_key: _vout.script_pub_key.clone(),
                    address: address_from_scriptpubkey(
                        &_vout.script_pub_key.as_ref().unwrap().hex.to_string()
                    ).unwrap_or_default(),
                })
                .collect();

            Transaction {
                index: _index as i64,
                height: blk.height.clone(),
                vout: vouts_parsed, // Assign the transformed vouts

                ..trans.clone()
            }
        })

        .collect();
    Ok(Transactions { transactions })
}

pub fn address_from_scriptpubkey(script_pub_key_hex: &str) -> Option<String> {
    // Decode the script from hex
    let hex_data = hex::decode(script_pub_key_hex).expect("Valid hex script");
    let script = Script::from_bytes(&hex_data);

    // Create a Bitcoin address from the public key script
    Address::from_script(script, Network::Bitcoin)
        .map(|address| address.to_string())
        .ok()
}
