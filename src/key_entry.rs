pub struct KeyEntry {
    // Max size of a key string is 2^16
    pub key: String,
    pub data_offset: u64,
    // Max size of data is 2^32
    pub data_size: u32
}

impl KeyEntry {
    pub fn new(key: String, data_offset: u64) -> Self {
        return KeyEntry {
            key,
            data_offset,
            data_size: 0
        }
    }

    pub fn with_size(key: String, data_offset: u64, data_size: u32) -> Self {
        return KeyEntry {
            key,
            data_offset,
            data_size
        }
    }
}