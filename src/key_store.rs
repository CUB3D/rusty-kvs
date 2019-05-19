use std::path::Path;
use crate::key_entry::KeyEntry;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::vec::Vec;

pub struct KeyStore {
    store_path: &'static Path,
    store: File
}

impl KeyStore {
    pub fn append_key(&mut self, key: &KeyEntry) {
        if let Err(e) = self.store.write_u16::<BigEndian>(key.key.len() as u16) {
            eprintln!("Unable to write key length to store: {:?}", e);
        }

        if let Err(e) = self.store.write(key.key.as_bytes()) {
            eprintln!("Unable to write key to store: {:?}", e);
        }

        if let Err(e) = self.store.write_u64::<BigEndian>(key.data_offset) {
            eprintln!("Unable to write offset to store: {:?}", e);
        }

        self.store.flush().expect("Unable to flush key store")
    }

    // Consider caching this in the store file, should be able to put in in a header
    pub fn get_size(&self) -> usize {
        return self.get_keys().len();
    }


    pub fn get_keys(&self) -> Vec<KeyEntry> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(self.store_path)
            .unwrap();

        let mut keys = Vec::<KeyEntry>::new();

        loop {
            match file.read_u16::<BigEndian>() {
                Ok(val) => {
                    let key_buf = vec![0u8; val as usize];
                    let mut boxed_buf = key_buf.into_boxed_slice();

                    // Read key
                    file.read_exact(&mut boxed_buf[..]).unwrap();
                    let offset = file.read_u64::<BigEndian>().unwrap();

                    keys.push(KeyEntry::new(
                        std::str::from_utf8(&boxed_buf).unwrap().to_string(),
                        offset
                    ))
                },
                Err(_e) => break
            }
        }

        keys
    }
}

impl Default for KeyStore {
    fn default() -> Self {
        let path = Path::new("./key_store");

        let store = OpenOptions::new()
            .append(true)
            .open(path)
            .unwrap();

        return KeyStore {
            store_path: path,
            store
        };
    }
}