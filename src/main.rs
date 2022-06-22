use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use simple_blockchain::now;

mod block;
mod blockchain;
mod hashtable;
mod transaction;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Charlie".to_owned(),
                    value: 100,
                },
            ],
        }],
        difficulty,
    );

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block.");

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Delta".to_owned(),
                    value: 567,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Elephant".to_owned(),
                        value: 678,
                    },
                    transaction::Output {
                        to_addr: "Greenwich".to_owned(),
                        value: 555,
                    },
                ],
            },
        ],
        difficulty,
    );

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    println!("last_hash {:?}", last_hash);

    blockchain
        .update_with_block(block)
        .expect("Failed to add block");
}
