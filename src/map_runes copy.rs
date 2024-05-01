use std::str::FromStr;

use bitcoin::absolute::LockTime;

use bitcoin::block::Version;
use bitcoin::hashes::sha256d::Hash;
use bitcoin::network::message_network::VersionMessage;
use bitcoin::{ Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, Txid, Witness };
use ordinals::{ Artifact, Runestone };

use crate::pb::btc::transaction_meta::v1::{ Block };
use crate::pb::btc::runes_meta::v1::{ Rune };
use bitcoin::{ locktime, opcodes, script::{ self, PushBytes }, TxOut };

#[substreams::handlers::map]
fn map_runes(blk: Block) -> Result<Rune, Vec<substreams::errors::Error>> {
    substreams::log::println("MAPPPPP RUNESSSSS");

    let mut rune_data: Rune = Rune::default();
    for trans in &blk.tx {
        substreams::log::println("Inside  tx map");

        let inputs = trans.vin.iter().map(|_vin| {
            // let wt = _vin.txinwitness
            //     .iter_mut()
            //     .map(|witness| witness.as_bytes())
            //     .collect();
            // let wt: [_; 4] = _vin.txinwitness
            //     .iter()
            //     .map(|witness| witness.as_bytes())
            //     .collect::<Vec<_>>()
            //     .try_into()
            //     .unwrap();
            TxIn {
                previous_output: OutPoint {
                    txid: Txid::from_raw_hash(
                        Hash::from_str(&trans.hash.clone().to_string()).unwrap()
                    ),
                    vout: _vin.vout,
                },
                script_sig: ScriptBuf::from_hex(
                    &_vin.script_sig.clone().unwrap_or_default().hex.to_string()
                ).unwrap(),
                sequence: Sequence::from_height(1), // Convert u32 to Sequence

                witness: Witness::new(),
                // Witness::from_slice(
                //     _vin.txinwitness
                //         .iter()
                //         .map(&(|witness| witness.as_bytes().to_vec()))
                //         .collect()
                // ), // Convert Vec<String> to Witness
            }
        });
        substreams::log::println(inputs.len().to_string());

        let outputs = trans.vout.iter().map(|_vout| {
            TxOut {
                script_pubkey: ScriptBuf::from_hex(
                    &_vout.script_pub_key.clone().unwrap().hex
                ).unwrap(),
                value: _vout.value.clone() as u64,
            }
        });
        substreams::log::println(outputs.len().to_string());

        let txnss = Transaction {
            version: 2,
            lock_time: locktime::absolute::LockTime::ZERO,
            input: inputs.collect(),
            output: outputs.collect(),
        };

        // let rune = Runestone::decipher(&txnss);
        if let rune = Runestone::decipher(&txnss) {
            if let Some(rune) = &rune {
                match rune {
                    Artifact::Cenotaph(cenotaph) => {
                        if let Some(mint) = cenotaph.etching {
                            substreams::log::println("cenotaph.etching");

                            // Do something with mint
                        }
                    }
                    Artifact::Runestone(runestone) => {
                        if let Some(mint) = runestone.etching {
                            substreams::log::println("runestone.etching");

                            // Do something with mint
                        }
                    }
                }
            }
            // let name = rune.etching.unwrap().symbol.clone();
            // let name_string = match name {
            //     Some(c) => c.to_string(),
            //     None => String::new(), // or any other default value or behavior you want
            // };
            // rune_data.name = name_string;
            // rune_data.symbol = "SS".to_string();
            // substreams::log::println("decoded rune");

            // Use the `name` variable here or perform any other operations
        } else {
            substreams::log::println("Unable to decode");

            substreams::log::info!("Unable to decode");

            // Handle the case where `decipher` returns `None`, if needed
        }
        // Do something with `element`
    }

    substreams::log::println("Outside  tx map");

    Ok(Rune { name: "SOME RUNE".to_string(), symbol: "SS".to_string() })
}

// let txouts = trans.vout
//     .iter()
//     .map(
//         vout |
//             ({
//                 TxOut {
//                     script_pubkey: vout.script_pubkey.clone(),
//                 }
//             })
//     )
//     .collect();
// let tx = Transaction {
//     input: Vec::new(),

//     output: vec![TxOut {
//         script_pubkey: trans.vout[0].script_pubkey.clone(),
//         value: 0,
//     }],
//     version: 2,
// };
