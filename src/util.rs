use std::borrow::Cow;
use encoding::{self, Encoding, DecoderTrap};

pub fn read_ascii_string(buffer: &[u8]) -> Result<String, Cow<'static, str>> {
    let string_end = buffer.iter().position(|&b| { b == 0 }).unwrap_or(buffer.len());
    return Ok(encoding::all::ASCII.decode(&buffer[0..string_end], DecoderTrap::Ignore)?);
}

pub fn read_utf_16le_string(buffer: &[u8]) -> Result<String, Cow<'static, str>> {
    let string_end = buffer[0..(buffer.len() / 2 * 2)]
        .chunks(2)
        .position(|v| { v[0] == 0 && v[1] == 0 })
        .map(|x| { 2 * x })
        .unwrap_or(buffer.len());
    return Ok(encoding::all::UTF_16LE.decode(&buffer[0..string_end], DecoderTrap::Ignore)?);
}
