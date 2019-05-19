use crate::key_entry::KeyEntry;
use crate::key_store::KeyStore;
use std::path::Path;
use std::fs;
use crate::value_entry::ValueEntry;
use crate::value_store::ValueStore;

pub struct KeyValueStore {
    pub key_store: KeyStore,
    pub value_store: ValueStore,
//    pub data_store: File,
    last_size: u64
}

impl KeyValueStore {
    pub fn store_data(&mut self, key: String, value: &[u8]) {

        let new_key = KeyEntry::new(key, self.last_size);

        let new_data = ValueEntry::with_data(value.len() as u32, value.to_vec());

        self.key_store.append_key(&new_key);
        self.value_store.append_value(&new_data);

        self.last_size += value.len() as u64;
    }

    pub fn get_data(&mut self, key: String) -> Vec<u8> {
        let keys = self.key_store.get_keys();

        let res = keys.iter().find(|c|
            c.key == key
        ).unwrap();

        let data = self.value_store.get_value_for_key(res);

        data.data
    }
}

impl Default for KeyValueStore {
    fn default() -> Self {
        let path = Path::new("./data_store");

        return KeyValueStore {
            key_store: KeyStore::default(),
            value_store: ValueStore::default(),
            last_size: fs::metadata(path).unwrap().len()
        };
    }
}