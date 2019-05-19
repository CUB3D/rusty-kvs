pub struct ValueEntry {
    // Max size of data is 2^32
    pub data_size: u32,
    pub data: Vec<u8>
}

impl ValueEntry {
    pub fn new(data_size: u32) -> Self {
        return ValueEntry {
            data_size,
            data: Vec::<u8>::with_capacity(data_size as usize)
        }
    }

    pub fn with_data(data_size: u32, data: Vec<u8>) -> Self {
        return ValueEntry {
            data_size,
            data
        }
    }
}