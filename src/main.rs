use encoding_rs::{Encoding, UTF_8};
use serde::{ser, Serialize};
use std::io::{self, Write};

struct EscapeNonAscii(String);

impl serde_json::ser::Formatter for EscapeNonAscii {
    fn write_string_fragment<W: ?Sized + Write>(
        &mut self,
        writer: &mut W,
        fragment: &str,
    ) -> io::Result<()> {
        for ch in fragment.chars() {
            if ch.is_ascii() {
                writer.write_all(ch.encode_utf8(&mut [0; 4]).as_bytes())?;
            } else {
                for escape in ch.encode_utf16(&mut [0; 2]) {
                    write!(writer, "\\u{:04x}", escape)?;
                }
            }
        }
        Ok(())
    }
}
pub fn convert(input: String) -> Vec<u8> {
    let encoding = UTF_8;
    let mut writer = Vec::new();
    let formatter = EscapeNonAscii(String::new());
    let mut ser = serde_json::Serializer::with_formatter(&mut writer, formatter);

    match encoding.decode(input.as_bytes()) {
        input_str => {
            input_str.serialize(&mut ser).unwrap();
        }
        _ => {
            panic!("Input string contains invalid characters");
        }
    };

    writer
}

fn main() {
    let value = "This is blah? !!!zh这么";

    let writer = convert(value.to_string());
    println!("{:?}", writer);
    if let Ok(json_string) = String::from_utf8(writer) {
        let res = serde_json::from_str::<serde_json::Value>(&json_string).unwrap();
        println!("{}", json_string.get(0..1).unwrap());
    }
}
