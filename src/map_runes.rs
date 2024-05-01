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
        // if let Artifact::Runestone(rune) = Runestone::decipher(&txnss).unwrap_or_default(Runestone) {
        if let rs = Runestone::decipher(&txnss) {
            if let Some(rs) = &rs {
                if let Artifact::Runestone(runestone) = rs {
                    if let Some(etching) = &runestone.etching {
                        if let Some(symbol) = &etching.rune {
                            substreams::log::println(symbol.to_string());
                        } else {
                            substreams::log::println("Symbol is None".to_string());
                        }
                    } else {
                        substreams::log::println("Etching is None".to_string());
                    }
                    // if let etching = runestone.etching {
                    //     // You have the Runestone artifact here, use it as needed
                    //     // substreams::log::println(runestone.edicts.len().to_string());
                    //     substreams::log::println(
                    //         runestone.etching.unwrap().symbol.unwrap().to_string()
                    //     );
                    // }
                }
            }
        } else {
            substreams::log::println("FAILED to decode".to_string());

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
