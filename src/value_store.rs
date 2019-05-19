use std::path::Path;
use std::fs::{File, OpenOptions};
use crate::value_entry::ValueEntry;
use byteorder::BigEndian;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::key_entry::KeyEntry;
use byteorder::{ReadBytesExt, WriteBytesExt};

pub struct ValueStore {
    store_path: &'static Path,
    store: File
}

impl ValueStore {
    pub fn append_value(&mut self, value: &ValueEntry) {
        if let Err(e) = self.store.write_u32::<BigEndian>(value.data_size) {
            eprintln!("Unable to write data size to store: {:?}", e);
        }

        if let Err(e) = self.store.write(&value.data.clone().into_boxed_slice()) {
            eprintln!("Unable to write data size to store: {:?}", e);
        }
    }

    pub fn get_value_for_key(&mut self, key: &KeyEntry) -> ValueEntry{
        self.store.seek(SeekFrom::Start(key.data_offset));

        let data_size = self.store.read_u32::<BigEndian>().unwrap();

        let data_buf = vec![0u8; data_size as usize];
        let mut boxed_buf = data_buf.into_boxed_slice();

        self.store.read_exact(&mut boxed_buf[..]).unwrap();

        let value = ValueEntry::with_data(data_size, boxed_buf.to_vec());

        self.store.seek(SeekFrom::End(0));

        return value;
    }

    // Consider caching this in the store file, should be able to put in in a header
    pub fn get_size(&self) -> usize {
        return self.get_values().len();
    }

    pub fn get_values(&self) -> Vec<ValueEntry> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(self.store_path)
            .unwrap();

        let mut values = Vec::<ValueEntry>::new();

        loop {
            match file.read_u32::<BigEndian>() {
                Ok(val) => {
                    let data_buf = vec![0u8; val as usize];
                    let mut boxed_buf = data_buf.into_boxed_slice();

                    file.read_exact(&mut boxed_buf[..]).unwrap();

                    values.push(ValueEntry::with_data(
                        val,
                        boxed_buf.to_vec()
                    ))
                },
                Err(e) => break
            }
        }

        values
    }
}

impl Default for ValueStore {
    fn default() -> Self {
        let path = Path::new("./data_store");

        let store = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .open(path)
            .unwrap();

        return ValueStore {
            store_path: path,
            store
        };
    }
}