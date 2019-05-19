#![feature(seek_convenience)]

mod key_entry;
mod key_value_store;
mod key_store;

fn main() {
    let data = "<html><p>Test123</p></html><html><p>Test123</p></html><html><p>Test123</p></html><html><p>Test123</p".as_bytes();


    let mut kvs = key_value_store::KeyValueStore::default();

//    for x in 0 .. 100000 {
        kvs.store_data("https://google.com".to_string(), data);
//    }

    let data = kvs.get_data("https://google.com".to_string());
    let str = data.into_boxed_slice();
    let val = std::str::from_utf8(&str).unwrap();
    println!("Returned: {:?}", val);
}
