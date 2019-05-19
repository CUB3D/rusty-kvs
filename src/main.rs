#![feature(seek_convenience)]

use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;

mod key_entry;
mod value_entry;
mod key_value_store;
mod key_store;
mod value_store;

fn main() {
    let mut kvs = key_value_store::KeyValueStore::default();

    let sites = Path::new("./ingest/").read_dir().expect("Directory not found");

    let mut added = 0;

    for file in sites {
        if let Ok(file) = file {
            let mut f = OpenOptions::new()
                .read(true)
                .open(file.path())
                .unwrap();

            println!("Size: {}", &file.metadata().unwrap().len());

            let data = vec![0u8; file.metadata().unwrap().len() as usize];
            let mut data_buf = data.into_boxed_slice();
            f.read_exact(&mut data_buf[..]).unwrap();


            kvs.store_data(file.file_name().to_str().unwrap().to_string(), &data_buf);
            added += 1;
        }
    }

    println!("Added {}", added);

    let mut fails = 0;

    for site in kvs.key_store.get_keys() {
        println!("Site: {:?}", &site.key);
        let data = kvs.value_store.get_value_for_key(&site);

        if let Some(data) = data {
            println!("Data: {} bytes", data.data_size);
            println!("----");
        } else {
            println!("Unable to read data");
            fails+=1;
        }
    }

    println!("There were {} fails", fails);


//    let data = kvs.get_data("https://google.com".to_string());
//    let str = data.into_boxed_slice();
//    let val = std::str::from_utf8(&str).unwrap();
//    println!("Returned: {:?}", val);
}
