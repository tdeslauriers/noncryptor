// Base64 lookup
const BASE_64_CHARS: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

pub fn encode(plain_text: &str) -> String {
    let utf_bytes = plain_text.as_bytes();
    let mut encoded = Vec::new();

    // Process plain_text in 3-byte chunks
    for i in (0..plain_text.len()).step_by(3) {
        let in_chunk = &utf_bytes[i..std::cmp::min(i + 3, utf_bytes.len())];
        let mut out_chunk = [0u8; 4];

        // Encoding logic for 3-byte chunks
        let indices = [
            (in_chunk[0] & 0xFC) >> 2,
            ((in_chunk[0] & 0x03) << 4) | ((in_chunk.get(1).copied().unwrap_or(0) & 0xF0) >> 4),
            ((in_chunk.get(1).copied().unwrap_or(0) & 0x0F) << 2)
                | ((in_chunk.get(2).copied().unwrap_or(0) & 0xC0) >> 6),
            in_chunk.get(2).copied().unwrap_or(0) & 0x3F,
        ];

        for (j, &index) in indices.iter().enumerate() {
            out_chunk[j] = BASE_64_CHARS[index as usize];
        }

        // Padding for chunks w/ length < 6
        match in_chunk.len() {
            1 => {
                out_chunk[2] = b'=';
                out_chunk[3] = b'=';
            }
            2 => out_chunk[3] = b'=',

            _ => (),
        }

        encoded.extend_from_slice(&out_chunk);
    }

    String::from_utf8(encoded).expect("Invalid UTF-8")
}

pub fn decode(encoded: &str) -> String {
    let mut decoded_bytes: Vec<u8> = Vec::new();
    let mut buffer = 0;
    let mut buffer_size = 0;

    for char in encoded.chars() {
        let value = BASE_64_CHARS
            .iter()
            .position(|&c| c == char as u8)
            .unwrap_or(0xFF);

        if value == 0xFF {
            continue; // skips invalid characters
        }

        buffer = (buffer << 6) | value as u32;
        buffer_size += 6;

        while buffer_size >= 8 {
            let byte = (buffer >> (buffer_size - 8)) as u8;
            decoded_bytes.push(byte);
            buffer_size -= 8;
        }
    }

    // handle any bits left in buffer
    if buffer_size >= 6 {
        let byte = (buffer << (8 - buffer_size)) as u8;
    }

    return String::from_utf8(decoded_bytes).expect("Invalid UFT-8");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_operators() {
        let test_clear_text = "dog".as_bytes().to_owned();

        let test_chunk = &test_clear_text[0..test_clear_text.len()];
        assert_eq!(test_chunk[0], 0b01100100);

        let base_char_1 = (test_chunk[0] & 0xFC) >> 2; // 0xFC = 11111100  -> zero's out the least significant 2 bits -> so the shift comes out correctly.
        assert_eq!(base_char_1, 0b00011001);

        let base_char_2_first = (test_chunk[0] & 0x03) << 4; // 0x03 = 00000011 -> zeros out all but the last two least significant bits so the left shift 4 comes out right.
        assert_eq!(base_char_2_first, 0b00000000);

        let base_char_2_second = (test_chunk[1] & 0xFC) >> 4; // 0xFC = 11110000
        assert_eq!(base_char_2_second, 0b00000110);

        let base_char_2 = base_char_2_first | base_char_2_second; // effective outcome of | operator is concatination
        assert_eq!(base_char_2, 0b00000110);

        let base_char_3_first = (test_chunk[1] & 0x0F) << 2; // 0x0F = 00001111
        assert_eq!(base_char_3_first, 0b00111100);

        let base_char_3_second = (test_chunk[2] & 0xC0) >> 6; // 0xC0 = 11000000
        assert_eq!(base_char_3_second, 0b00000001);

        let base_char_3 = base_char_3_first | base_char_3_second;
        assert_eq!(base_char_3, 0b00111101);

        let base_char_4 = test_chunk[2] & 0x3F; //0x3F = 00111111
        assert_eq!(base_char_4, 0b00100111);

        print!("{:08b}\n", base_char_2_first)
    }

    #[test]
    fn test_base64_indices() {
        let test_clear_text = "atomic dog... is a song that i really like.";

        println!("{}", encode(&test_clear_text));
    }

    #[test]
    fn test_base64_decode() {
        let encoded = encode("Atomic Dog...");
        println!("{}", encoded);
        println!("{}", decode(&encoded));

        // test from outside encoding:
        println!(
            "{}",
            decode("Ym93IHdvdyB3b3cgeWlwcHkgeW8geWlwcHkgeWF5Li4uCg==")
        );

        assert_eq!(
            "Do the dog catcher...",
            decode(&encode("Do the dog catcher..."))
        );
        // encoded by bash: echo -n "Do the dog catcher..." | base64
        assert_eq!(
            "Do the dog catcher...",
            decode("RG8gdGhlIGRvZyBjYXRjaGVyLi4u")
        );
    }
}
