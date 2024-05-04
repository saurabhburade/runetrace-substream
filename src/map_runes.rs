use std::str::FromStr;

use bitcoin::absolute::LockTime;

use bitcoin::block::Version;
use bitcoin::consensus::{ deserialize, params, Decodable };
use bitcoin::hashes::sha256d::Hash;
use bitcoin::network::message_network::VersionMessage;
use bitcoin::{
    Address,
    Amount,
    OutPoint,
    PublicKey,
    Script,
    ScriptBuf,
    Sequence,
    Transaction,
    TxIn,
    Txid,
    Witness,
};
use ordinals::{ Artifact, Edict, Etching, Runestone };

use crate::address::address_from_scriptpubkey;
use crate::pb::btc::transaction_meta::v1::{ Block };
use crate::pb::btc::runes_meta::v1::{ Rune, RunestoneBuf, RunestoneBufs };
use bitcoin::{ locktime, opcodes, script::{ self, PushBytes }, TxOut };
use hex::FromHex;

use substreams::pb::substreams::store_delta::Operation;

#[substreams::handlers::map]
fn map_runes(blk: Block) -> Result<RunestoneBufs, Vec<substreams::errors::Error>> {
    substreams::log::println("MAPPPPP RUNESSSSS");

    let mut rune_data: Rune = Rune::default();
    let mut runes: Vec<RunestoneBuf> = Vec::new();

    for (index, trans) in blk.tx.iter().enumerate() {
        let mut currentRune = RunestoneBuf::default();

        // let raw_tx = hex::tes!(
        //     "0200000000010102628409a0b48dcbef8f612a9ee791e797ff50f61b71616ed761875e5c9de5670000000000ffffffff0222020000000000002251209fa6f1741e417f9217bc920024be14f9528c5bdf32ce188a6689aec49ac8386b0000000000000000096a5d0614011400160001404769f48a0ee5316869132824e8ca0247c93afc0b097b2804624fdb3000fe7ef9d1ee7646ceeda115f3c0c3c8e4aeb89caac72bd4928a02b4c0bb1dcd0f8b30f900000000"
        // );

        // 512054624c8dd9216a5958455f9a424dd5208f599b5b88a9c62bd9055a4705083ca4
        // let addr = address_from_scriptpubkey(
        //     &"512054624c8dd9216a5958455f9a424dd5208f599b5b88a9c62bd9055a4705083ca4".to_string()
        // ).unwrap_or_default();
        // let script = Script::from_bytes(&hex_data);

        // // 4769f48a0ee5316869132824e8ca0247c93afc0b097b2804624fdb3000fe7ef9d1ee7646ceeda115f3c0c3c8e4aeb89caac72bd4928a02b4c0bb1dcd0f8b30f9
        // substreams::log::println(Address::p2wsh(script, bitcoin::Network::Bitcoin).to_string());
        // substreams::log::println(addr.to_string());

        let inputs = trans.vin.iter().map(|_vin| {
            // let wt: Witness = Witness::from_slice(
            //     &_vin.txinwitness
            //         .iter()
            //         .map(|witness| witness.as_bytes().to_vec())
            //         .collect::<Vec<Vec<u8>>>()
            //         .as_slice()
            // );

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

                witness: Witness::from_slice(
                    &_vin.txinwitness
                        .iter()
                        .map(|witness| witness.as_bytes().to_vec())
                        .collect::<Vec<Vec<u8>>>()
                        .as_slice()
                ), // Convert Vec<String> to Witness
                // Witness::new(),
            }
        });
        // substreams::log::println(inputs.len().to_string());

        let outputs = trans.vout.iter().map(|_vout| {
            TxOut {
                script_pubkey: ScriptBuf::from_hex(
                    &_vout.script_pub_key.clone().unwrap().hex
                ).unwrap(),
                value: _vout.value.clone() as u64,
            }
        });
        // substreams::log::println(outputs.len().to_string());

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
                        // substreams::log::println("RUNE::ID");
                        // substreams::log::println(blk.height.to_string());

                        // substreams::log::println(index.to_string());
                        // substreams::log::println(trans.txid.to_string());
                        // substreams::log::println(
                        //     etching.rune
                        //         .as_ref()
                        //         .map(|r| r.to_string())
                        //         .unwrap_or_default()
                        //         .to_string()
                        // );

                        currentRune.etching = Some(crate::pb::btc::runes_meta::v1::Etching {
                            divisibility: etching.divisibility.map(|d| d as i32),
                            rune: etching.rune.as_ref().map(|r| r.to_string()), // Wrap the value in Some
                            spacers: Some(etching.spacers.unwrap_or_default().to_string()), // Wrap the value in Some
                            symbol: Some(etching.symbol.unwrap_or_default().to_string()), // Wrap the value in Some
                            turbo: Some(etching.turbo), // Wrap the value in Some
                            premine: etching.premine.map(|p| p.to_string()), // Convert Option<u128> to Option<String>
                            terms: None, // Add the missing field 'terms'
                            supply: Some(etching.supply().unwrap_or_default().to_string()), // Add the missing field 'terms'
                            id: Some(crate::pb::btc::runes_meta::v1::RuneId {
                                block: blk.height.to_string(), // Unwrap the Option value
                                tx: index.to_string(), // Add the missing field 'cap'
                            }),
                        });
                        if let Some(rune) = etching.rune.as_ref().map(|r| r.to_string()) {
                            // substreams::log::println(rune);``
                        }
                        if let Some(terms) = &etching.terms {
                            currentRune.etching = Some(crate::pb::btc::runes_meta::v1::Etching {
                                divisibility: etching.divisibility.map(|d| d as i32),
                                rune: etching.rune.as_ref().map(|r| r.to_string()), // Wrap the value in Some
                                spacers: Some(etching.spacers.unwrap_or_default().to_string()), // Wrap the value in Some
                                symbol: Some(etching.symbol.unwrap_or_default().to_string()), // Wrap the value in Some
                                turbo: Some(etching.turbo), // Wrap the value in Some
                                premine: etching.premine.map(|p| p.to_string()), // Convert Option<u128> to Option<String>
                                terms: Some(crate::pb::btc::runes_meta::v1::Terms {
                                    amount: Some(terms.amount.unwrap_or_default().to_string()),
                                    cap: Some(terms.cap.unwrap_or_default().to_string()), // Add the missing field 'cap'
                                }), // Add the missing field 'terms'
                                supply: Some(etching.supply().unwrap_or_default().to_string()), // Add the missing field 'terms',
                                id: Some(crate::pb::btc::runes_meta::v1::RuneId {
                                    block: blk.height.to_string(), // Unwrap the Option value
                                    tx: index.to_string(), // Add the missing field 'cap'
                                }),
                            });
                            // if let Some(cap) = &terms.cap {
                            //     // substreams::log::println(cap.to_string());
                            // }
                        } else {
                            // substreams::log::println("Symbol is None".to_string());
                        }
                    }
                    if let Some(pointer) = &runestone.pointer {
                        currentRune.pointer = pointer.to_string();
                        // substreams::log::println("pointer ::: AMT START".to_string());
                        // substreams::log::println(pointer.to_string());
                        // substreams::log::println(trans.txid.to_string());
                        // substreams::log::println("pointer ::: AMT END".to_string());
                    }
                    if let Some(mint) = &runestone.mint {
                        currentRune.mint = Some(crate::pb::btc::runes_meta::v1::RuneId {
                            block: mint.block.to_string(),
                            tx: mint.tx.to_string(),
                        });
                        // substreams::log::println("MINT ::: AMT START".to_string());
                        // substreams::log::println(mint.block.to_string());
                        // substreams::log::println(mint.tx.to_string());
                        // substreams::log::println(trans.txid.to_string());
                        // substreams::log::println("MINT ::: AMT END".to_string());
                    }
                    let mut edicts: Vec<crate::pb::btc::runes_meta::v1::Edict> = Vec::new();
                    for Edict { id, amount, output } in runestone.edicts.iter().copied() {
                        let newEdict = crate::pb::btc::runes_meta::v1::Edict {
                            amount: amount.to_string(),
                            output: output.to_string(),
                            id: Some(crate::pb::btc::runes_meta::v1::RuneId {
                                block: id.block.to_string(),
                                tx: id.tx.to_string(),
                            }),
                        };
                        edicts.push(newEdict);
                        // substreams::log::println("EDICTTT ::: AMT START".to_string());
                        // substreams::log::println(id.to_string());
                        // substreams::log::println(amount.to_string());
                        // substreams::log::println(output.to_string());
                        // substreams::log::println("EDICTTT ::: AMT END".to_string());
                    }
                    // if let etching = runestone.etching {
                    //     // You have the Runestone artifact here, use it as needed
                    //     // substreams::log::println(runestone.edicts.len().to_string());
                    //     substreams::log::println(
                    //         runestone.etching.unwrap().symbol.unwrap().to_string()
                    //     );
                    // }
                    currentRune.edicts = edicts;
                    runes.push(currentRune);
                }
                if let Artifact::Cenotaph(cenotaph) = rs {
                    substreams::log::println("cenotaph  :::  START".to_string());

                    if let Some(etching) = &cenotaph.etching {
                        substreams::log::println("cenotaph.etching  ::: AMT START".to_string());

                        substreams::log::println(&etching.0.to_string());
                    }
                    if let Some(flaw) = &cenotaph.flaw {
                        substreams::log::println("cenotaph.flaw  ::: AMT START".to_string());

                        substreams::log::println(&flaw.to_string());
                    }
                    if let Some(mint) = &cenotaph.mint {
                        substreams::log::println("cenotaph.mint  ::: AMT START".to_string());

                        substreams::log::println(&mint.to_string());
                    }
                }
            }
        } else {
            substreams::log::println("FAILED to decode".to_string());

            // Handle the case where `decipher` returns `None`, if needed
        }
        // Do something with `element`
    }

    // substreams::log::println("Outside  tx map");

    // let mut runes: Vec<RunestoneBuf> = Vec::new();
    Ok(RunestoneBufs { runestone_bufs: runes })
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
