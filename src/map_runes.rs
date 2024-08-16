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
use crate::pb;
// use crate::pb::btc::transaction_meta::v1::{ Block };
use crate::pb::btc::runes_meta::v1::{ Rune, RunestoneBuf, RunestoneBufs };
use crate::pb::sf::bitcoin::r#type::v1::Block;
use bitcoin::{ locktime, opcodes, script::{ self, PushBytes }, TxOut };
use hex::FromHex;
use crate::pb::ordinals::v1::{ self as ord_proto, Inscription };

use substreams::pb::substreams::store_delta::Operation;

#[substreams::handlers::map]
fn map_runes(blk: Block) -> Result<RunestoneBufs, Vec<substreams::errors::Error>> {
    substreams::log::println("MAPPPPP RUNESSSSS".to_string());

    let mut rune_data: Rune = Rune::default();
    let mut runes: Vec<RunestoneBuf> = Vec::new();

    for (index, trans) in blk.tx.iter().enumerate() {
        let mut currentRune = RunestoneBuf::default();
        let inputs = trans.vin.iter().map(|_vin| {

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
                sequence: Sequence::from_height(1), 

                witness: Witness::from_slice(
                    &_vin.txinwitness
                        .iter()
                        .map(|witness| witness.as_bytes().to_vec())
                        .collect::<Vec<Vec<u8>>>()
                        .as_slice()
                ), 
            }
        });


        let outputs = trans.vout.iter().map(|_vout| {
            TxOut {
                script_pubkey: ScriptBuf::from_hex(
                    &_vout.script_pub_key.clone().unwrap().hex
                ).unwrap(),
                value: _vout.value.clone() as u64,
            }
        });


        let txnss = Transaction {
            version: 2,
            lock_time: locktime::absolute::LockTime::ZERO,
            input: inputs.collect(),
            output: outputs.collect(),
        };

    
        if let rs = Runestone::decipher(&txnss) {
            if let Some(rs) = &rs {
                substreams::log::println("rs  :::  START".to_string());
                if let Artifact::Runestone(runestone) = rs {
                    if let Some(etching) = &runestone.etching {
                        substreams::log::println("RUNE::ID");
                        substreams::log::println(
                            blk.height.to_string() + "" + index.to_string().as_str()
                        );

                        substreams::log::println(trans.txid.to_string());
                        substreams::log::println(
                            etching.rune
                                .as_ref()
                                .map(|r| r.to_string())
                                .unwrap_or_default()
                                .to_string()
                        );
                        substreams::log::println(etching.premine.unwrap_or_default().to_string());

                        currentRune.etching = Some(crate::pb::btc::runes_meta::v1::Etching {
                            divisibility: etching.divisibility.map(|d| d as i32),
                            rune: etching.rune.as_ref().map(|r| r.to_string()), 
                            spacers: Some(etching.spacers.unwrap_or_default().to_string()), 
                            symbol: Some(etching.symbol.unwrap_or_default().to_string()), 
                            turbo: Some(etching.turbo), 
                            premine: etching.premine.map(|p| p.to_string()), 
                            terms: None, 
                            supply: Some(etching.supply().unwrap_or_default().to_string()), 
                            id: Some(crate::pb::btc::runes_meta::v1::RuneId {
                                block: blk.height.to_string(), 
                                tx: index.to_string(), 
                            }),
                        });
                        if let Some(rune) = etching.rune.as_ref().map(|r| r.to_string()) {

                        }
                        if let Some(terms) = &etching.terms {
                            currentRune.etching = Some(crate::pb::btc::runes_meta::v1::Etching {
                                divisibility: etching.divisibility.map(|d| d as i32),
                                rune: etching.rune.as_ref().map(|r| r.to_string()), 
                                spacers: Some(etching.spacers.unwrap_or_default().to_string()), 
                                symbol: Some(etching.symbol.unwrap_or_default().to_string()), 
                                turbo: Some(etching.turbo), 
                                premine: etching.premine.map(|p| p.to_string()), 
                                terms: Some(crate::pb::btc::runes_meta::v1::Terms {
                                    amount: Some(terms.amount.unwrap_or_default().to_string()),
                                    cap: Some(terms.cap.unwrap_or_default().to_string()), 
                                }), 
                                supply: Some(etching.supply().unwrap_or_default().to_string()), ,
                                id: Some(crate::pb::btc::runes_meta::v1::RuneId {
                                    block: blk.height.to_string(), 
                                    tx: index.to_string(), 
                                }),
                            });

                        } else {
                            // substreams::log::println("Symbol is None".to_string());
                        }
                    }
                    if let Some(pointer) = &runestone.pointer {
                        currentRune.pointer = pointer.to_string();
                    }
                    if let Some(mint) = &runestone.mint {
                        currentRune.mint = Some(crate::pb::btc::runes_meta::v1::RuneId {
                            block: mint.block.to_string(),
                            tx: mint.tx.to_string(),
                        });

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
    
                    }

                
                    currentRune.edicts = edicts;
                }
                if let Artifact::Cenotaph(cenotaph) = rs {
                    substreams::log::println("cenotaph  :::  START".to_string());

                    if let Some(etching) = &cenotaph.etching {
                        substreams::log::println("cenotaph.etching  ::: AMT START".to_string());

                        substreams::log::println(&etching.0.to_string());
                        currentRune.pointer = "0".to_string();
                    }
                    if let Some(flaw) = &cenotaph.flaw {
                        substreams::log::println("cenotaph.flaw  ::: AMT START".to_string());

                        substreams::log::println(&flaw.to_string());
                        currentRune.pointer = "0".to_string();
                    }
                    if let Some(mint) = &cenotaph.mint {
                        substreams::log::println("cenotaph.mint  ::: AMT START".to_string());

                        substreams::log::println(&mint.to_string());
                        currentRune.pointer = "0".to_string();
                    }
                }
                substreams::log::println("ETCHING::STOP :: ".to_string() + ":::");
                substreams::log::println(
                    currentRune
                        .clone()
                        .etching.unwrap_or_default()
                        .terms.unwrap_or_default()
                        .cap.unwrap_or_default()
                        .to_string() +
                        ":::" +
                        &currentRune
                            .clone()
                            .etching.unwrap_or_default()
                            .id.unwrap_or_default()
                            .block.to_string() +
                        ":::" +
                        &currentRune
                            .clone()
                            .etching.unwrap_or_default()
                            .id.unwrap_or_default()
                            .tx.to_string()
                );

                runes.push(currentRune);
            }
        } else {
            substreams::log::println("FAILED to decode".to_string());


        }

    }

    Ok(RunestoneBufs { runestone_bufs: runes })
}

pub fn map_runes2(
    blk: Block
) -> Result<Vec<ord_proto::RunestoneBuf>, Vec<substreams::errors::Error>> {
    substreams::log::println("MAPPPPP RUNESSSSS");

    let mut rune_data: Rune = Rune::default();
    let mut runes: Vec<ord_proto::RunestoneBuf> = Vec::new();

    for (index, trans) in blk.tx.iter().enumerate() {
        let mut currentRune = ord_proto::RunestoneBuf::default();


        let inputs = trans.vin.iter().map(|_vin| {

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
                sequence: Sequence::from_height(1), 

                witness: Witness::from_slice(
                    &_vin.txinwitness
                        .iter()
                        .map(|witness| witness.as_bytes().to_vec())
                        .collect::<Vec<Vec<u8>>>()
                        .as_slice()
                ), 
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
                substreams::log::println("rs  :::  START".to_string());

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
    Ok(runes)
}

pub fn txn_to_rune(
    trans: pb::sf::bitcoin::r#type::v1::Transaction,
    blkHeight: u64,
    index: usize
) -> Result<ord_proto::RunestoneBuf, Vec<substreams::errors::Error>> {
    let mut currentRune = ord_proto::RunestoneBuf::default();

    let inputs = &trans.vin.iter().map(|_vin| {
        TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_str(&trans.hash.clone().to_string()).unwrap()),
                vout: _vin.vout,
            },
            script_sig: ScriptBuf::from_hex(
                &_vin.script_sig.clone().unwrap_or_default().hex.to_string()
            ).unwrap(),
            sequence: Sequence::from_height(1), 

            witness: Witness::from_slice(
                &_vin.txinwitness
                    .iter()
                    .map(|witness| witness.as_bytes().to_vec())
                    .collect::<Vec<Vec<u8>>>()
                    .as_slice()
            ), 
        }
    });

    let outputs = trans.vout.iter().map(|_vout| {
        TxOut {
            script_pubkey: ScriptBuf::from_hex(&_vout.script_pub_key.clone().unwrap().hex).unwrap(),
            value: _vout.value.clone() as u64,
        }
    });


    let txnss = Transaction {
        version: 2,
        lock_time: locktime::absolute::LockTime::ZERO,
        input: inputs.clone().collect(),
        output: outputs.clone().collect(),
    };


    if let rs = Runestone::decipher(&txnss) {
        if let Some(rs) = &rs {
            if let Artifact::Runestone(runestone) = rs {
                if let Some(etching) = &runestone.etching {

                    currentRune.etching = Some(ord_proto::Etching {
                        divisibility: etching.divisibility.map(|d| d as i32),
                        rune: etching.rune.as_ref().map(|r| r.to_string()), 
                        spacers: Some(etching.spacers.unwrap_or_default().to_string()), 
                        symbol: Some(etching.symbol.unwrap_or_default().to_string()), 
                        turbo: Some(etching.turbo), 
                        premine: etching.premine.map(|p| p.to_string()), 
                        terms: None, 
                        supply: Some(etching.supply().unwrap_or_default().to_string()), 
                        id: Some(ord_proto::RuneId {
                            block: blkHeight.to_string(), 
                            tx: (index + 1).to_string(),
                        }),
                    });

                    if let Some(terms) = &etching.terms {
                        currentRune.etching = Some(ord_proto::Etching {
                            divisibility: etching.divisibility.map(|d| d as i32),
                            rune: etching.rune.as_ref().map(|r| r.to_string()), 
                            spacers: Some(etching.spacers.unwrap_or_default().to_string()), 
                            symbol: Some(etching.symbol.unwrap_or_default().to_string()), 
                            turbo: Some(etching.turbo), 
                            premine: etching.premine.map(|p| p.to_string()), 
                            terms: Some(ord_proto::Terms {
                                amount: Some(terms.amount.unwrap_or_default().to_string()),
                                cap: Some(terms.cap.unwrap_or_default().to_string()), 
                            }), 
                            supply: Some(etching.supply().unwrap_or_default().to_string()), ,
                            id: Some(ord_proto::RuneId {
                                block: blkHeight.to_string(), 
                                tx: (index + 1).to_string(), 
                            }),
                        });

                    } else {
                        // substreams::log::println("Symbol is None".to_string());
                    }
                }
                if let Some(pointer) = &runestone.pointer {
                    currentRune.pointer = pointer.to_string();
                }
                if let Some(mint) = &runestone.mint {
                    currentRune.mint = Some(ord_proto::RuneId {
                        block: mint.block.to_string(),
                        tx: mint.tx.to_string(),
                    });

                }
                let mut edicts: Vec<ord_proto::Edict> = Vec::new();
                for Edict { id, amount, output } in runestone.edicts.iter().copied() {
                    let newEdict: ord_proto::Edict = ord_proto::Edict {
                        amount: amount.to_string(),
                        output: output.to_string(),
                        id: Some(ord_proto::RuneId {
                            block: id.block.to_string(),
                            tx: id.tx.to_string(),
                        }),
                    };
                    edicts.push(newEdict);

                }
                currentRune.edicts = edicts;
                // runes.push(currentRune);
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

    }
    Ok(currentRune)
    // Ok(Some(currentRune))
}
