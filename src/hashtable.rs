// use super::*;
use simple_blockchain::IHash;

pub trait Hashtable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> IHash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &Self::bytes(&self))
    }
}
