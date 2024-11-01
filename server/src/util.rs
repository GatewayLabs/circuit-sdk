use tokio::io::{self, Result};

pub fn prepare(mut data: Vec<u8>) -> Vec<u8> {
    // Get the length of the data and convert it to a 32-bit little-endian representation
    let length = data.len() as u32;
    let mut length_bytes = length.to_le_bytes().to_vec();

    // Prepend the length bytes to the data
    length_bytes.append(&mut data);
    length_bytes
}

pub fn extract(data: &[u8]) -> Result<(u32, Vec<u8>)> {
    // Ensure there are at least 4 bytes for the length header
    if data.len() < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Data does not contain a length header.",
        ));
    }

    // Extract the first 4 bytes as the length (little-endian)
    let length_bytes: [u8; 4] = data[..4].try_into().expect("slice with incorrect length");
    let length = u32::from_le_bytes(length_bytes);

    // Extract the remaining data based on the length
    let payload = data[4..].to_vec();

    Ok((length, payload))
}
