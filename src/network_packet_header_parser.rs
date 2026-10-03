fn parse_header_safe(bytes: &[u8]) -> Result<(u16, u32), std::array::TryFromSliceError> {
    let magic = u16::from_be_bytes(bytes[0..2].try_into()?);
    let length = u32::from_be_bytes(bytes[2..6].try_into()?);
    Ok((magic, length))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Magic: 0x1234 (4660), Length: 0x000000FF (255)
    let raw_packet: [u8; 6] = [0x12, 0x34, 0x00, 0x00, 0x00, 0xFF];

    let (magic, length) = parse_header_safe(&raw_packet)?;

    assert_eq!(magic, 0x1234);
    assert_eq!(length, 255);
    println!("Final Integer Challenge Passed!");

    Ok(())
}