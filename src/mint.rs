//! A library for generating the chain's genesis block.

use event::Event;
use transaction::Transaction;
use signature::{KeyPair, KeyPairUtil, PublicKey};
use entry::Entry;
use entry::create_entry;
use hash::{hash, Hash};
use ring::rand::SystemRandom;
use untrusted::Input;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mint {
    pub pkcs8: Vec<u8>,
    pubkey: PublicKey,
    pub tokens: i64,
}

impl Mint {
    pub fn new(tokens: i64) -> Self {
        let rnd = SystemRandom::new();
        let pkcs8 = KeyPair::generate_pkcs8(&rnd).unwrap().to_vec();
        let keypair = KeyPair::from_pkcs8(Input::from(&pkcs8)).unwrap();
        let pubkey = keypair.pubkey();
        Mint {
            pkcs8,
            pubkey,
            tokens,
        }
    }

    pub fn seed(&self) -> Hash {
        hash(&self.pkcs8)
    }

    pub fn keypair(&self) -> KeyPair {
        KeyPair::from_pkcs8(Input::from(&self.pkcs8)).unwrap()
    }

    pub fn pubkey(&self) -> PublicKey {
        self.pubkey
    }

    pub fn create_events(&self) -> Vec<Event> {
        let keypair = self.keypair();
        let tr = Transaction::new(&keypair, self.pubkey(), self.tokens, self.seed());
        vec![Event::Transaction(tr)]
    }

    pub fn create_entries(&self) -> Vec<Entry> {
        let e0 = create_entry(&self.seed(), 0, vec![]);
        let e1 = create_entry(&e0.id, 0, self.create_events());
        vec![e0, e1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::verify_slice;

    #[test]
    fn test_create_events() {
        let mut events = Mint::new(100).create_events().into_iter();
        if let Event::Transaction(tr) = events.next().unwrap() {
            assert_eq!(tr.from, tr.to);
        }
        assert_eq!(events.next(), None);
    }

    #[test]
    fn test_verify_entries() {
        let entries = Mint::new(100).create_entries();
        assert!(verify_slice(&entries, &entries[0].id));
    }
}
