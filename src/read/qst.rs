use std::io::{self, Cursor, Read, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use prs;
use read::{self, ReadError};
use read::dat::{self, DatFile};
use read::bin::{self, BinFile};
use util::read_ascii_string;

// A .qst file contains two headers describing the embedded files, a .dat and a .bin file in that order.
pub struct QstFile {
    pub dat: DatFile,
    pub bin: BinFile
}

// Low level read method for .qst files.
pub fn read<T: Read + Seek>(data: &mut T) -> read::Result<QstFile> {
    // Read headers.
    // A .qst file contains two 88-byte headers that describe the embedded .dat and .bin files.
    let mut dat_size = 0;
    let mut bin_size = 0;

    for i in 0..2 {
        data.seek(SeekFrom::Current(44))?;
        let mut buffer = [0; 16];
        data.read_exact(&mut buffer)?;
        let file_name = read_ascii_string(&buffer)?;
        let size = data.read_u32::<LittleEndian>()?;
        data.seek(SeekFrom::Current(24))?;

        if i == 0 && file_name.ends_with(".dat") {
            dat_size = size;
        } else if i == 1 && file_name.ends_with(".bin") {
            bin_size = size;
        }
    }

    if dat_size == 0 || bin_size == 0 {
        return Err(ReadError::InvalidData);
    }

    // Extract the embedded files and decompress them.
    let (dat_file_data, bin_file_data) = extract_file_data(data, dat_size as usize, bin_size as usize)?;
    let decompressed_dat = decompress(&dat_file_data[..])?;
    let decompressed_bin = decompress(&bin_file_data[..])?;

    // Read the embedded files.
    return Ok(QstFile {
        dat: dat::read(&mut Cursor::new(decompressed_dat))?,
        bin: bin::read(&mut Cursor::new(decompressed_bin))?
    });
}

fn extract_file_data<T: Read + Seek>(data: &mut T, dat_size: usize, bin_size: usize) -> read::Result<(Vec<u8>, Vec<u8>)> {
    // .dat and .bin files are interleaved in 1056 byte blocks.
    // Each block has a 24 byte header, 1024 data segment and an 8 byte trailer.
    let mut dat_data = vec![0; dat_size];
    let mut bin_data = vec![0; bin_size];
    let mut dat_read = 0;
    let mut bin_read = 0;

    while dat_read < dat_size || bin_read < bin_size {
        let start_pos = data.seek(SeekFrom::Current(8))? - 8;
        let mut buffer = [0; 16];
        data.read_exact(&mut buffer)?;
        let file_name = read_ascii_string(&buffer)?;
        data.seek(SeekFrom::Current(1024))?;
        let size = data.read_u32::<LittleEndian>()? as usize;

        if size <= 1024 {
            if file_name.ends_with(".dat") {
                data.seek(SeekFrom::Current(-1028))?;
                data.read_exact(&mut dat_data[dat_read..(dat_read + size) as usize])?;
                dat_read += size;
                data.seek(SeekFrom::Current(1028 - size as i64))?;
            } else if file_name.ends_with(".bin") {
                data.seek(SeekFrom::Current(-1028))?;
                data.read_exact(&mut bin_data[bin_read..(bin_read + size) as usize])?;
                bin_read += size;
                data.seek(SeekFrom::Current(1028 - size as i64))?;
            }
        } else {
            return Err(ReadError::InvalidData);
        }

        let end_pos = data.seek(SeekFrom::Current(4))?;
        debug_assert!(start_pos == end_pos - 1056);
    }

    if dat_read != dat_size || bin_read != bin_size {
        return Err(ReadError::InvalidData);
    }

    return Ok((dat_data, bin_data));
}

fn decompress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decompressed = Cursor::new(Vec::with_capacity(4 * data.len()));
    prs::decompress(&mut Cursor::new(data), &mut decompressed)?;
    return Ok(decompressed.into_inner());
}
