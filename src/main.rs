extern crate hex;

use std::env;

fn calculate_key(seed: u32, root: u32) -> u32 {
    let seed_regrouped = 
        (seed & 0xff) << 0x10 |
        (seed & 0xff00) >> 8 |
        (seed & 0xff0000) << 8 |
        (seed & 0xff000000) >> 0x10;
    let num_of_rotations = ((seed_regrouped & 7) + 2) as u8;
    let mut root_rotated = root;
    for _ in 0..num_of_rotations {
        let bits_xored = 
            get_bit(root_rotated, 0) ^
            get_bit(root_rotated, 7) ^
            get_bit(root_rotated, 0x11) ^
            get_bit(root_rotated, 0x1a);
        root_rotated = (root_rotated & 0x7fffffff) | ((bits_xored as u32) << 0x1f);
        root_rotated = rotate_u32_right(root_rotated);
    }
    let seed_high = seed_regrouped >> 0x10;
    let seed_low = seed_regrouped & 0xffff;
    let mut tmp_1 = seed_high as u16;
    let mut tmp_2 = seed_low as u16;
    for i in 0..2 {
        tmp_1 = tmp_1 ^ tmp_2;
        tmp_1 = rotate_u16_left(tmp_1, num_of_rotations);
        let tmp_3: u16;
        if i == 0 {
            tmp_3 = (tmp_1 as u32 + (root_rotated & 0xffff)) as u16;
        } else {
            tmp_3 = (tmp_1 as u32 + (root_rotated >> 0x10)) as u16;
        }
        tmp_1 = tmp_2;
        tmp_2 = tmp_3;
    }
    return 
        (tmp_2 as u32) << 0x18 |
        (tmp_2 as u32 & 0xff00) << 8 |
        (((tmp_1 as u32) << 0x10) & 0xff0000) >> 0x10 |
        (((tmp_1 as u32) << 0x10) & 0xff000000) >> 0x10;
}

fn get_bit(value: u32, bit: u8) -> u8 {
    return ((value & (1 << bit)) >> bit) as u8;
}

fn rotate_u32_right(value: u32) -> u32 {
    if value & 1 == 0 {
        return value >> 1;
    } else {
        return (value >> 1) | 0x80000000;
    }
}

fn rotate_u16_left(value: u16, rotations: u8) -> u16 {
    let mut result = value;
    for _ in 0..rotations {
        if result & 0x8000 == 0 {
            result <<= 1;
        } else {
            result = (result << 1) | 1;
        }
    }
    result
}

fn get_4bytes_seed(seed: u64) -> u32 {
    return (
        (seed & 0xff00_0000_0000_0000) >> 0x38 |
        (seed & 0x0000_ff00_0000_0000) >> 0x20 |
        (seed & 0x0000_0000_ff00_0000) >> 0x8 |
        (seed & 0x0000_0000_0000_ff00) << 0x10
    ) as u32;
}

fn seed_key(seed: u64, root: u32) -> Vec<u8> {
    let key = calculate_key(get_4bytes_seed(seed), root);
    let mut result = Vec::new();
    result.push(0x07);  // access level
    result.append(&mut key.to_be_bytes().to_vec());
    result.append(&mut [0xFF, 0xFF].to_vec());  // dongle id
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", hex::encode(seed_key(u64::from_str_radix(args[1].as_str(), 16).unwrap(), 0x3913B1FF)).to_uppercase());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_209_1056() {
        // Request (get seed): 27 01
        let root = 0x3913B1FF;
        assert_eq!(seed_key(0x847d94ea99279f96, root), hex::decode("07F74DD2C8FFFF").unwrap());
        assert_eq!(seed_key(0, root), hex::decode("0743A0EC7FFFFF").unwrap());
        // Response (send key): 27 02 07 43 A0 EC 7F FF FF
    }

    #[test]
    fn test_209_1057() {
        let root = 0x4532F3EF;
        assert_eq!(seed_key(0x0000_0000_000_0001, root), hex::decode("073A85BCFBFFFF").unwrap());
    }

    #[test]
    fn test_211_0095() {
        let root = 0x2a58122f;
        assert_eq!(seed_key(0, root), hex::decode("07C2BC048BFFFF").unwrap());
    }

    #[test]
    fn test_211_00A9() {
        let root = 0x6A04B0BF;
        assert_eq!(seed_key(0x847d94ea99279f96, root), hex::decode("076AE396C3FFFF").unwrap());
    }

    #[test]
    fn test_171_AEJ_07_1() {
        // Response here differs from 211/209
        
        // Request: 27 05
        let root = 0x10CC5C1;  // access level 05
        assert_eq!(calculate_key(get_4bytes_seed(0x0000_0000_000_0001), root), 0x03C63170);
        // Response: 27 06 03 C6 31 70
    }
}
