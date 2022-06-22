// use super::*;
use crate::hashtable;
use hashtable::Hashtable;
use simple_blockchain::{u64_bytes, Address, IHash};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashtable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

#[derive(Clone)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<IHash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<IHash>>()
    }

    pub fn output_hashes(&self) -> HashSet<IHash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<IHash>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashtable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes
    }
}
