pub fn bit_counter(number: u64, little_endian: bool) -> ([u8; 8], u32) {
    if little_endian {
        (number.to_le_bytes(), number.count_ones())
    } else {
        (number.to_be_bytes(), number.count_ones())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_counter() {
        let bit_tuple = bit_counter(0xFF_FF_FF_FF_FF_FF_FF_FF,true);
        assert_eq!(bit_tuple.0, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(bit_tuple.1, 64);
        let bit_tuple = bit_counter(0xAB_CD_EF_AB_CD_EF_AB_CD,false);
        assert_eq!(bit_tuple.0, [0xAB, 0xCD, 0xEF, 0xAB, 0xCD, 0xEF, 0xAB, 0xCD]);
        assert_eq!(bit_tuple.1, 0xAB_CD_EF_AB_CD_EF_AB_CD_u64.count_ones());
    }
}
