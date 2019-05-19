use std::path::Path;
use crate::key_entry::KeyEntry;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::mem;
use std::intrinsics::write_bytes;
use std::vec::Vec;

pub struct KeyStore {
    store_path: &'static Path,
    store: File
}

impl KeyStore {
    pub fn append_key(&mut self, key: &KeyEntry) {
        if let Err(e) = self.store.write_u32::<BigEndian>(key.key.len() as u32) {
            eprintln!("Unable to write key length to store: {:?}", e);
        }

        if let Err(e) = self.store.write(key.key.as_bytes()) {
            eprintln!("Unable to write key to store: {:?}", e);
        }

        if let Err(e) = self.store.write_u64::<BigEndian>(key.data_offset) {
            eprintln!("Unable to write offset to store: {:?}", e);
        }

        if let Err(e) = self.store.write_u32::<BigEndian>(key.data_size) {
            eprintln!("Unable to write data size to store: {:?}", e);
        }
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
            match file.read_u32::<BigEndian>() {
                Ok(val) => {
                    let key_buf = vec![0u8; val as usize];
                    let mut boxed_buf = key_buf.into_boxed_slice();

                    let key = file.read_exact(&mut boxed_buf[..]).unwrap();
                    let offset = file.read_u64::<BigEndian>().unwrap();
                    let size = file.read_u32::<BigEndian>().unwrap();

                    keys.push(KeyEntry::with_size(
                        std::str::from_utf8(&boxed_buf).unwrap().to_string(),
                        offset,
                        size
                    ))
                },
                Err(e) => break
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
            store: store
        };
    }
}