use std::io::{Read, Seek, SeekFrom};
use read;
use util::read_utf_16le_string;

// A .bin file contains meta information and the assembly code.
pub struct BinFile {
    pub quest_name: String,
    pub short_description: String
}

// Low level read method for .bin files.
pub fn read<T: Read + Seek>(data: &mut T) -> read::Result<BinFile> {
    data.seek(SeekFrom::Current(0x18))?;

    let mut name_buffer = [0; 64];
    data.read_exact(&mut name_buffer)?;
    let mut short_description_buffer = [0; 256];
    data.read_exact(&mut short_description_buffer)?;

    return Ok(BinFile {
        quest_name: read_utf_16le_string(&name_buffer[..])?,
        short_description: read_utf_16le_string(&short_description_buffer[..])?
    });
}
