use std::collections::HashMap;

fn strxor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let len = std::cmp::min(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .take(len)
        .map(|(x, y)| x ^ y)
        .collect()
}

fn letter_position(s: &[u8]) -> Vec<usize> {
    s.iter()
        .enumerate()
        .filter(|(_, &c)| c.is_ascii_alphabetic())
        .map(|(i, _)| i)
        .collect()
}

fn find_space(
    ciphertexts: &[Vec<u8>],
    num_cipher: usize,
    key_size: usize,
    threshold_value: usize,
) -> HashMap<usize, Vec<usize>> {
    let mut space_possible: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    let mut space_position: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..num_cipher {
        let mut space_xor: Vec<Vec<usize>> = Vec::new();
        for j in 0..num_cipher {
            if i != j {
                let xor_result = strxor(&ciphertexts[i], &ciphertexts[j]);
                let positions = letter_position(&xor_result);
                space_xor.push(positions);
            }
        }
        space_possible.insert(i, space_xor);
    }

    for i in 0..num_cipher {
        let mut spa: Vec<usize> = Vec::new();
        for position in 0..key_size {
            let mut count = 0;
            for space in space_possible.get(&i).unwrap().iter() {
                if space.contains(&position) {
                    count += 1;
                }
            }
            if count > threshold_value {
                spa.push(position);
            }
        }
        space_position.insert(i, spa);
    }

    space_position
}
fn calculate_key(
    ciphertexts: &[Vec<u8>],
    num_cipher: usize,
    key_size: usize,
    threshold_value: usize,
) -> Vec<u8> {
    let mut key = vec![0u8; key_size];
    let space_positions = find_space(ciphertexts, num_cipher, key_size, threshold_value);

    for (i, space_pos) in space_positions.iter() {
        for &pos in space_pos {
            if pos < ciphertexts[*i].len() {
                key[pos] = ciphertexts[*i][pos] ^ b' ';
            }
        }
    }

    key
}

fn decrypt_with_key(ciphertext: &[u8], key: &[u8]) -> String {
    let decrypted = strxor(ciphertext, key);
    let result: String = decrypted
        .iter()
        .map(|&c| {
            if c.is_ascii_alphabetic() || c == b' ' {
                c as char
            } else {
                '0'
            }
        })
        .collect();

    result
}

fn main() {
    let ciphertexts: Vec<Vec<u8>> = vec![
        vec![
            0x31, 0x5c, 0x4e, 0xea, 0xa8, 0xb5, 0xf8, 0xaa, 0xf9, 0x17, 0x41, 0x45, 0xbf, 0x43,
            0xe1, 0x78, 0x4b, 0x8f, 0xa0, 0x0d, 0xc7, 0x1d, 0x88, 0x5a, 0x80, 0x4e, 0x5e, 0xe9,
            0xfa, 0x40, 0xb1, 0x63, 0x49, 0xc1, 0x46, 0xfb, 0x77, 0x8c, 0xdf, 0x2d, 0x3a, 0xff,
            0x02, 0x1d, 0xff, 0xf5, 0xb4, 0x03, 0xb5, 0x10, 0xd0, 0xd0, 0x45, 0x54, 0x68, 0xae,
            0xb9, 0x86, 0x22, 0xb1, 0x37, 0xda, 0xe8, 0x57, 0x55, 0x3c, 0xcd, 0x88, 0x83, 0xa7,
            0xbc, 0x37, 0x52, 0x0e, 0x06, 0xe5, 0x15, 0xd2, 0x2c, 0x95, 0x4e, 0xba, 0x50, 0x25,
            0xb8, 0xcc, 0x57, 0xee, 0x59, 0x41, 0x8c, 0xe7, 0xdc, 0x6b, 0xc4, 0x15, 0x56, 0xbd,
            0xb3, 0x6b, 0xbc, 0xa3, 0xe8, 0x77, 0x43, 0x01, 0xfb, 0xca, 0xa3, 0xb8, 0x3b, 0x22,
            0x08, 0x09, 0x56, 0x09, 0x87, 0x81, 0x5f, 0x65, 0x28, 0x67, 0x64, 0x70, 0x3d, 0xe0,
            0xf3, 0xd5, 0x24, 0x40, 0x0a, 0x19, 0xb1, 0x59, 0x61, 0x0b, 0x11, 0xef, 0x3e,
        ],
        vec![
            0x23, 0x4c, 0x02, 0xec, 0xbb, 0xfb, 0xaf, 0xa3, 0xed, 0x18, 0x51, 0x0a, 0xbd, 0x11,
            0xfa, 0x72, 0x4f, 0xcd, 0xa2, 0x01, 0x8a, 0x1a, 0x83, 0x42, 0xcf, 0x06, 0x4b, 0xbd,
            0xe5, 0x48, 0xb1, 0x2b, 0x07, 0xdf, 0x44, 0xba, 0x71, 0x91, 0xd9, 0x60, 0x6e, 0xf4,
            0x08, 0x1f, 0xfd, 0xe5, 0xad, 0x46, 0xa5, 0x06, 0x9d, 0x9f, 0x7f, 0x54, 0x3b, 0xed,
            0xb9, 0xc8, 0x61, 0xbf, 0x29, 0xc7, 0xe2, 0x05, 0x13, 0x2e, 0xda, 0x93, 0x82, 0xb0,
            0xbc, 0x2c, 0x5c, 0x4b, 0x45, 0xf9, 0x19, 0xcf, 0x3a, 0x9f, 0x1c, 0xb7, 0x41, 0x51,
            0xf6, 0xd5, 0x51, 0xf4, 0x48, 0x0c, 0x82, 0xb2, 0xcb, 0x24, 0xcc, 0x5b, 0x02, 0x8a,
            0xa7, 0x6e, 0xb7, 0xb4, 0xab, 0x24, 0x17, 0x1a, 0xb3, 0xcd, 0xad, 0xb8, 0x35, 0x6f,
        ],
        vec![
            0x32, 0x51, 0x0b, 0xa9, 0xa7, 0xb2, 0xbb, 0xa9, 0xb8, 0x00, 0x5d, 0x43, 0xa3, 0x04,
            0xb5, 0x71, 0x4c, 0xc0, 0xbb, 0x0c, 0x8a, 0x34, 0x88, 0x4d, 0xd9, 0x13, 0x04, 0xb8,
            0xad, 0x40, 0xb6, 0x2b, 0x07, 0xdf, 0x44, 0xba, 0x6e, 0x9d, 0x8a, 0x23, 0x68, 0xe5,
            0x1d, 0x04, 0xe0, 0xe7, 0xb2, 0x07, 0xb7, 0x0b, 0x9b, 0x82, 0x61, 0x11, 0x2b, 0xac,
            0xb6, 0xc8, 0x66, 0xa2, 0x32, 0xdf, 0xe2, 0x57, 0x52, 0x7d, 0xc2, 0x93, 0x98, 0xf5,
            0xf3, 0x25, 0x1a, 0x0d, 0x47, 0xe5, 0x03, 0xc6, 0x6e, 0x93, 0x5d, 0xe8, 0x12, 0x30,
            0xb5, 0x9b, 0x7a, 0xfb, 0x5f, 0x41, 0xaf, 0xa8, 0xd6, 0x61, 0xcb,
        ],
        vec![
            0x32, 0x51, 0x0b, 0xa9, 0xaa, 0xb2, 0xa8, 0xa4, 0xfd, 0x06, 0x41, 0x4f, 0xb5, 0x17,
            0xb5, 0x60, 0x5c, 0xc0, 0xaa, 0x0d, 0xc9, 0x1a, 0x89, 0x08, 0xc2, 0x06, 0x4b, 0xa8,
            0xad, 0x5e, 0xa0, 0x6a, 0x02, 0x90, 0x56, 0xf4, 0x7a, 0x8a, 0xd3, 0x30, 0x6e, 0xf5,
            0x02, 0x1e, 0xaf, 0xe1, 0xac, 0x01, 0xa8, 0x11, 0x97, 0x84, 0x7a, 0x5c, 0x68, 0xa1,
            0xb7, 0x87, 0x69, 0xa3, 0x7b, 0xc8, 0xf4, 0x57, 0x54, 0x32, 0xc1, 0x98, 0xcc, 0xb4,
            0xef, 0x63, 0x59, 0x02, 0x56, 0xe3, 0x05, 0xcd, 0x3a, 0x95, 0x44, 0xee, 0x41, 0x60,
            0xea, 0xd4, 0x5a, 0xef, 0x52, 0x04, 0x89, 0xe7, 0xda, 0x7d, 0x83, 0x54, 0x02, 0xbc,
            0xa6, 0x70, 0xbd, 0xa8, 0xeb, 0x77, 0x52, 0x00, 0xb8, 0xda, 0xbb, 0xba, 0x24, 0x6b,
            0x13, 0x0f, 0x04, 0x0d, 0x8e, 0xc6, 0x44, 0x7e, 0x2c, 0x76, 0x7f, 0x3d, 0x30, 0xed,
            0x81, 0xea, 0x2e, 0x4c, 0x14, 0x04, 0xe1, 0x31, 0x5a, 0x10, 0x10, 0xe7, 0x22, 0x9b,
            0xe6, 0x63, 0x6a, 0xaa,
        ],
        vec![
            0x3f, 0x56, 0x1b, 0xa9, 0xad, 0xb4, 0xb6, 0xeb, 0xec, 0x54, 0x42, 0x4b, 0xa3, 0x17,
            0xb5, 0x64, 0x41, 0x8f, 0xac, 0x0d, 0xd3, 0x5f, 0x8c, 0x08, 0xd3, 0x1a, 0x1f, 0xe9,
            0xe2, 0x4f, 0xe5, 0x68, 0x08, 0xc2, 0x13, 0xf1, 0x7c, 0x81, 0xd9, 0x60, 0x7c, 0xee,
            0x02, 0x1d, 0xaf, 0xe1, 0xe0, 0x01, 0xb2, 0x1a, 0xde, 0x87, 0x7a, 0x5e, 0x68, 0xbe,
            0xa8, 0x8d, 0x61, 0xb9, 0x3a, 0xc5, 0xee, 0x0d, 0x56, 0x2e, 0x8e, 0x95, 0x82, 0xf5,
            0xef, 0x37, 0x5f, 0x0a, 0x4a, 0xe2, 0x0e, 0xd8, 0x6e, 0x93, 0x5d, 0xe8, 0x12, 0x30,
            0xb5, 0x9b, 0x73, 0xfb, 0x43, 0x02, 0xcd, 0x95, 0xd7, 0x70, 0xc6, 0x5b, 0x40, 0xaa,
            0xa0, 0x65, 0xf2, 0xa5, 0xe3, 0x3a, 0x5a, 0x0b, 0xb5, 0xdc, 0xab, 0xa4, 0x37, 0x22,
            0x13, 0x0f, 0x04, 0x2f, 0x8e, 0xc8, 0x5b, 0x7c, 0x20, 0x70,
        ],
        vec![
            0x32, 0x51, 0x0b, 0xfb, 0xac, 0xfb, 0xb9, 0xbe, 0xfd, 0x54, 0x41, 0x5d, 0xa2, 0x43,
            0xe1, 0x69, 0x5e, 0xca, 0xbd, 0x58, 0xc5, 0x19, 0xcd, 0x4b, 0xd2, 0x06, 0x1b, 0xbd,
            0xe2, 0x4e, 0xb7, 0x6a, 0x19, 0xd8, 0x4a, 0xba, 0x34, 0xd8, 0xde, 0x28, 0x7b, 0xe8,
            0x4d, 0x07, 0xe7, 0xe9, 0xa3, 0x0e, 0xe7, 0x14, 0x97, 0x9c, 0x7e, 0x11, 0x23, 0xa8,
            0xbd, 0x98, 0x22, 0xa3, 0x3e, 0xca, 0xf5, 0x12, 0x47, 0x2e, 0x8e, 0x8f, 0x8d, 0xb3,
            0xf9, 0x63, 0x5c, 0x19, 0x49, 0xe6, 0x40, 0xc6, 0x21, 0x85, 0x4e, 0xba, 0x0d, 0x79,
            0xec, 0xcf, 0x52, 0xff, 0x11, 0x12, 0x84, 0xb4, 0xcc, 0x61, 0xd1, 0x19, 0x02, 0xae,
            0xbc, 0x66, 0xf2, 0xb2, 0xe4, 0x36, 0x43, 0x4e, 0xac, 0xc0, 0xab, 0xa9, 0x38, 0x22,
            0x0b, 0x08, 0x48, 0x00, 0xc2, 0xca, 0x4e, 0x69, 0x35, 0x22, 0x64, 0x35, 0x73, 0xb2,
            0xc4, 0xce, 0x35, 0x05, 0x0b, 0x0c, 0xf7, 0x74, 0x20, 0x1f, 0x0f, 0xe5, 0x2a, 0xc9,
            0xf2, 0x6d, 0x71, 0xb6, 0xcf, 0x61, 0xa7, 0x11, 0xcc, 0x22, 0x9f, 0x77, 0xac, 0xe7,
            0xaa, 0x88, 0xa2, 0xf1, 0x99, 0x83, 0x12, 0x2b, 0x11, 0xbe, 0x87, 0xa5, 0x9c, 0x35,
            0x5d, 0x25, 0xf8, 0xe4,
        ],
        vec![
            0x32, 0x51, 0x0b, 0xfb, 0xac, 0xfb, 0xb9, 0xbe, 0xfd, 0x54, 0x41, 0x5d, 0xa2, 0x43,
            0xe1, 0x69, 0x5e, 0xca, 0xbd, 0x58, 0xc5, 0x19, 0xcd, 0x4b, 0xd9, 0x0f, 0x1f, 0xa6,
            0xea, 0x5b, 0xa4, 0x7b, 0x01, 0xc9, 0x09, 0xba, 0x76, 0x96, 0xcf, 0x60, 0x6e, 0xf4,
            0x0c, 0x04, 0xaf, 0xe1, 0xac, 0x0a, 0xa8, 0x14, 0x8d, 0xd0, 0x66, 0x59, 0x2d, 0xed,
            0x9f, 0x87, 0x74, 0xb5, 0x29, 0xc7, 0xea, 0x12, 0x5d, 0x29, 0x8e, 0x88, 0x83, 0xf5,
            0xe9, 0x30, 0x5f, 0x4b, 0x44, 0xf9, 0x15, 0xcb, 0x2b, 0xd0, 0x5a, 0xf5, 0x13, 0x73,
            0xfd, 0x9b, 0x4a, 0xf5, 0x11, 0x03, 0x9f, 0xa2, 0xd9, 0x6f, 0x83, 0x41, 0x4a, 0xaa,
            0xf2, 0x61, 0xbd, 0xa2, 0xe9, 0x7b, 0x17, 0x0f, 0xb5, 0xcc, 0xe2, 0xa5, 0x3e, 0x67,
            0x5c, 0x15, 0x4c, 0x0d, 0x96, 0x81, 0x59, 0x69, 0x34, 0x77, 0x7e, 0x22, 0x75, 0xb3,
            0x81, 0xce, 0x2e, 0x40, 0x58, 0x2a, 0xfe, 0x67, 0x65, 0x0b, 0x13, 0xe7, 0x22, 0x87,
            0xff, 0x22, 0x70, 0xab, 0xcf, 0x73, 0xbb, 0x02, 0x89, 0x32, 0x83, 0x6f, 0xbd, 0xec,
            0xfe, 0xce, 0xe0, 0xa3, 0xb8, 0x94, 0x47, 0x3c, 0x1b, 0xbe, 0xb6, 0xb4, 0x91, 0x3a,
            0x53, 0x6c, 0xe4, 0xf9, 0xb1, 0x3f, 0x1e, 0xff, 0xf7, 0x1e, 0xa3, 0x13, 0xc8, 0x66,
            0x1d, 0xd9, 0xa4, 0xce,
        ],
        vec![
            0x31, 0x5c, 0x4e, 0xea, 0xa8, 0xb5, 0xf8, 0xbf, 0xfd, 0x11, 0x15, 0x5e, 0xa5, 0x06,
            0xb5, 0x60, 0x41, 0xc6, 0xa0, 0x0c, 0x8a, 0x08, 0x85, 0x4d, 0xd2, 0x1a, 0x4b, 0xbd,
            0xe5, 0x4c, 0xe5, 0x68, 0x01, 0xd9, 0x43, 0xba, 0x70, 0x8b, 0x8a, 0x35, 0x74, 0xf4,
            0x0c, 0x00, 0xff, 0xf9, 0xe0, 0x0f, 0xa1, 0x43, 0x9f, 0xd0, 0x65, 0x43, 0x27, 0xa3,
            0xbf, 0xc8, 0x60, 0xb9, 0x2f, 0x89, 0xee, 0x04, 0x13, 0x2e, 0xcb, 0x92, 0x98, 0xf5,
            0xfd, 0x2d, 0x5e, 0x4b, 0x45, 0xe4, 0x0e, 0xcc, 0x3b, 0x9d, 0x59, 0xe9, 0x41, 0x7d,
            0xf7, 0xc9, 0x5b, 0xba, 0x41, 0x0e, 0x9a, 0xa2, 0xca, 0x24, 0xc5, 0x47, 0x4d, 0xa2,
            0xf2, 0x76, 0xba, 0xa3, 0xac, 0x32, 0x59, 0x18, 0xb2, 0xda, 0xad, 0xa4, 0x3d, 0x67,
            0x12, 0x15, 0x04, 0x41, 0xc2, 0xe0, 0x4f, 0x65, 0x65, 0x51, 0x7f, 0x31, 0x7d, 0xa9,
            0xd3,
        ],
        vec![
            0x27, 0x19, 0x46, 0xf9, 0xbb, 0xb2, 0xae, 0xad, 0xec, 0x11, 0x18, 0x41, 0xa8, 0x1a,
            0xbc, 0x30, 0x0e, 0xca, 0xa0, 0x1b, 0xd8, 0x06, 0x9d, 0x5c, 0xc9, 0x10, 0x05, 0xe9,
            0xfe, 0x4a, 0xad, 0x6e, 0x04, 0xd5, 0x13, 0xe9, 0x6d, 0x99, 0xde, 0x25, 0x69, 0xbc,
            0x5e, 0x50, 0xee, 0xec, 0xa7, 0x09, 0xb5, 0x0a, 0x8a, 0x98, 0x7f, 0x42, 0x64, 0xed,
            0xb6, 0x89, 0x6f, 0xb5, 0x37, 0xd0, 0xa7, 0x16, 0x13, 0x2d, 0xdc, 0x93, 0x8f, 0xb0,
            0xf8, 0x36, 0x48, 0x0e, 0x06, 0xed, 0x0f, 0xcd, 0x6e, 0x97, 0x59, 0xf4, 0x04, 0x62,
            0xf9, 0xcf, 0x57, 0xf4, 0x56, 0x41, 0x86, 0xa2, 0xc1, 0x77, 0x8f, 0x15, 0x43, 0xef,
            0xa2, 0x70, 0xbd, 0xa5, 0xe9, 0x33, 0x42, 0x1c, 0xbe, 0x88, 0xa4, 0xa5, 0x22, 0x22,
            0x19, 0x0f, 0x47, 0x1e, 0x9b, 0xd1, 0x5f, 0x65, 0x2b, 0x65, 0x3b, 0x70, 0x71, 0xae,
            0xc5, 0x9a, 0x27, 0x05, 0x08, 0x1f, 0xfe, 0x72, 0x65, 0x1d, 0x08, 0xf8, 0x22, 0xc9,
            0xed, 0x6d, 0x76, 0xe4, 0x8b, 0x63, 0xab, 0x15, 0xd0, 0x20, 0x85, 0x73, 0xa7, 0xee,
            0xf0, 0x27,
        ],
        vec![
            0x46, 0x6d, 0x06, 0xec, 0xe9, 0x98, 0xb7, 0xa2, 0xfb, 0x1d, 0x46, 0x4f, 0xed, 0x2c,
            0xed, 0x76, 0x41, 0xdd, 0xaa, 0x3c, 0xc3, 0x1c, 0x99, 0x41, 0xcf, 0x11, 0x0a, 0xbb,
            0xf4, 0x09, 0xed, 0x39, 0x59, 0x80, 0x05, 0xb3, 0x39, 0x9c, 0xcf, 0xaf, 0xb6, 0x1d,
            0x03, 0x15, 0xfc, 0xa0, 0xa3, 0x14, 0xbe, 0x13, 0x8a, 0x9f, 0x32, 0x50, 0x3b, 0xed,
            0xac, 0x80, 0x67, 0xf0, 0x3a, 0xdb, 0xf3, 0x57, 0x5c, 0x3b, 0x8e, 0xdc, 0x9b, 0xa7,
            0xf5, 0x37, 0x53, 0x05, 0x41, 0xab, 0x0f, 0x9f, 0x3c, 0xd0, 0x4f, 0xf5, 0x0d, 0x66,
            0xf1, 0xd5, 0x59, 0xba, 0x52, 0x0e, 0x89, 0xa2, 0xcb, 0x2a, 0x83,
        ],
        vec![
            0x32, 0x51, 0x0b, 0xa9, 0xba, 0xbe, 0xbb, 0xbe, 0xfd, 0x00, 0x15, 0x47, 0xa8, 0x10,
            0xe6, 0x71, 0x49, 0xca, 0xee, 0x11, 0xd9, 0x45, 0xcd, 0x7f, 0xc8, 0x1a, 0x05, 0xe9,
            0xf8, 0x5a, 0xac, 0x65, 0x0e, 0x90, 0x52, 0xba, 0x6a, 0x8c, 0xd8, 0x25, 0x7b, 0xf1,
            0x4d, 0x13, 0xe6, 0xf0, 0xa8, 0x03, 0xb5, 0x4f, 0xde, 0x9e, 0x77, 0x47, 0x2d, 0xbf,
            0xf8, 0x9d, 0x71, 0xb5, 0x7b, 0xdd, 0xef, 0x12, 0x13, 0x36, 0xcb, 0x85, 0xcc, 0xb8,
            0xf3, 0x31, 0x5f, 0x4b, 0x52, 0xe3, 0x01, 0xd1, 0x6e, 0x9f, 0x52, 0xf9, 0x04,
        ],
    ];

    let key_size = 100;
    let num_cipher = ciphertexts.len();
    let threshold_value = 6;

    // Calculate the key using the ciphertexts
    let key = calculate_key(&ciphertexts, num_cipher, key_size, threshold_value);
    println!("Key: {:?}", String::from_utf8_lossy(&key));

    // Decrypt the target ciphertext (e.g., ciphertexts[10])
    let decrypted_message = decrypt_with_key(&ciphertexts[10], &key);
    println!("Decrypted Message: {}", decrypted_message);

    // XOR known plaintext with ciphertext to reveal key (if desired)
    let known_plaintext =
        b"The secret message is When using a stream cipher never use the key more than once";
    let extracted_key = strxor(&ciphertexts[10], known_plaintext);
    println!(
        "Extracted Key: {:?}",
        String::from_utf8_lossy(&extracted_key)
    );
}
