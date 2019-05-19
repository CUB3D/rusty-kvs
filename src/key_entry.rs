pub struct KeyEntry {
    // Max size of a key string is 2^16
    pub key: String,
    pub data_offset: u64,
}

impl KeyEntry {
    pub fn new(key: String, data_offset: u64) -> Self {
        return KeyEntry {
            key,
            data_offset,
        }
    }
}