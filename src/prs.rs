use std::io::{self, Read, Write, Seek, SeekFrom};

pub fn decompress<S: Read, D: Read + Write + Seek>(src: &mut S, dst: &mut D) -> io::Result<()> {
    let mut cxt = Context {
        src: src,
        dst: dst,
        flags: 0,
        bit_pos: 0,
        buf: [0; 256]
    };

    loop {
        if cxt.read_flag_bit()? == 1 {
            // Single byte copy.
            cxt.copy_byte()?;
        } else {
            // Multi byte copy.
            let mut size: u16;
            let mut offset: i16;

            if cxt.read_flag_bit()? == 0 {
                // Short copy.
                size = (cxt.read_flag_bit()? as u16) << 1;
                size |= cxt.read_flag_bit()? as u16;
                size += 2;

                offset = cxt.read_byte()? as i16;
                offset |= 0xFF00u32 as i16;
            } else {
                // Long copy or end of file.
                offset = cxt.read_short()? as i16;

                // Two zero bytes implies that this is the end of the file.
                if offset == 0 {
                    return Ok(());
                }

                // Do we need to read a size byte, or is it encoded in what we already have?
                size = offset as u16 & 0b111;
                offset >>= 3;

                if size == 0 {
                    size = cxt.read_byte()? as u16;
                    size += 1;
                } else {
                    size += 2;
                }

                offset |= 0xE000u32 as i16;
            }

            cxt.offset_copy(offset, size)?;
        }
    }
}

struct Context<'a, S: 'a + Read, D: 'a + Read + Write + Seek> {
    src: &'a mut S,
    dst: &'a mut D,
    flags: u8,
    bit_pos: u8,
    buf: [u8; 256]
}

impl<'a, S: Read, D: Read + Write + Seek> Context<'a, S, D> {
    fn read_flag_bit(&mut self) -> io::Result<u8> {
        // Fetch a new flag byte when the previous byte has been processed.
        if self.bit_pos == 0 {
            self.src.read_exact(&mut self.buf[0..1])?;
            self.flags = self.buf[0];
            self.bit_pos = 8;
        }

        let rv = (self.flags) & 1;
        self.flags >>= 1;
        self.bit_pos -= 1;
        return Ok(rv);
    }

    fn copy_byte(&mut self) -> io::Result<()> {
        self.src.read_exact(&mut self.buf[0..1])?;
        self.dst.write_all(&self.buf[0..1])?;
        return Ok(());
    }

    fn read_byte(&mut self) -> io::Result<u8> {
        self.src.read_exact(&mut self.buf[0..1])?;
        return Ok(self.buf[0]);
    }

    fn read_short(&mut self) -> io::Result<u16> {
        // In little endian format.
        self.src.read_exact(&mut self.buf[0..2])?;
        return Ok(((self.buf[1] as u16) << 8) | (self.buf[0] as u16));
    }

    fn offset_copy(&mut self, offset: i16, size: u16) -> io::Result<()> {
        debug_assert!(-8192 <= offset && offset <= 0);
        debug_assert!(1 <= size && size <= 256);

        let offset = offset as i64;
        let size = size as usize;

        // The size can be larger than -offset, in that case we copy -offset bytes size/-offset times.
        let buf_size = ::std::cmp::min(-offset as usize, size);
        let buf = &mut self.buf[0..buf_size];
        self.dst.seek(SeekFrom::Current(offset))?;
        self.dst.read_exact(buf)?;
        self.dst.seek(SeekFrom::Current(-offset - buf_size as i64))?;

        for _ in 0..(size / buf_size) {
            self.dst.write_all(buf)?;
        }

        self.dst.write_all(&buf[0..(size % buf_size)])?;

        return Ok(());
    }
}
