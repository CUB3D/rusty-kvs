use crate::key_entry::KeyEntry;
use crate::key_store::KeyStore;
use std::path::Path;
use std::fs::{OpenOptions, File};
use std::io::{Write, Seek, SeekFrom, Read};
use std::fs;

pub struct KeyValueStore {
    pub key_store: KeyStore,
    pub data_store: File,
    last_size: u64
}

impl KeyValueStore {
    pub fn store_data(&mut self, key: String, value: &[u8]) {

        let new_key = KeyEntry::with_size(key, self.last_size, value.len() as u32);

        self.key_store.append_key(&new_key);

        if let Err(e) = self.data_store.write(value) {
            eprintln!("Unable to write data for key: {:?}", e);
        }

        self.last_size += value.len() as u64;
    }

    pub fn get_data(&mut self, key: String) -> Vec<u8> {
        let keys = self.key_store.get_keys();

        let res = keys.iter().find(|c|
            c.key == key
        ).unwrap();

        println!("Reading from offset: {}, {}", &res.data_offset, &res.data_size);

        self.data_store.seek(SeekFrom::Start(res.data_offset)).unwrap();

        let mut data = vec![0u8; res.data_size as usize];

        self.data_store.read_exact(&mut data).unwrap();

        self.data_store.seek(SeekFrom::End(0));

        data
    }
}

impl Default for KeyValueStore {
    fn default() -> Self {
        let path = Path::new("./data_store");

        let mut data = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .open(path)
            .unwrap();

        return KeyValueStore {
            key_store: KeyStore::default(),
            data_store: data,
            last_size: fs::metadata(path).unwrap().len()
        };
    }
}