use crate::block;
use crate::block::Block;
use crate::hashtable::Hashtable;
use simple_blockchain::IHash;
use std::collections::HashSet;
// use std::fmt::Error;

#[derive(Debug)]
pub enum BlockValidatorErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<IHash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidatorErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidatorErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidatorErr::InvalidHash);
        } else if i != 0 {
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidatorErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidatorErr::MismatchedPreviousHash);
            }
        } else {
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidatorErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidatorErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<IHash> = HashSet::new();
            let mut block_created: HashSet<IHash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidatorErr::InvalidInput);
                }

                let input_value = transaction.input_value();

                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidatorErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidatorErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}
