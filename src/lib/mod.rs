use byteorder::{LittleEndian, ReadBytesExt};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

use std::str;

trait ChunkIO<T> {
    fn new(buf: &[u8]) -> T;
    fn cursor_over<'a>(&self, cursor: Cursor<Vec<u8>>) -> Result<Cursor<Vec<u8>>, Box<dyn Error>>;
}

struct RiffHeader<'a> {
    id: &'a str,
    size: u32,
    format: &'a str,
}

impl<'a> ChunkIO<RiffHeader<'a>> for RiffHeader<'a> {
    fn new(buf: &[u8]) -> RiffHeader<'a> {
        if buf.len() != 12 {
            panic!("Riff header should be 12 bytes");
        }
        let id = match str::from_utf8(&buf[..4]) {
            Ok("RIFF") => "RIFF",
            _ => panic!("Riff header id can only be 'RIFF'"),
        };
        let size = read_buffer_as_u32(&buf[4..8]);
        let format = match str::from_utf8(&buf[8..]) {
            Ok("WAVE") => "WAVE",
            _ => panic!("Riff header format can only be 'WAVE'"),
        };
        RiffHeader { id, format, size }
    }

    fn cursor_over(&self, mut cursor: Cursor<Vec<u8>>) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
        // let mut c = cursor.to_owned();
        cursor.write(self.id.as_bytes())?;
        cursor.write(&self.size.to_le_bytes())?;
        cursor.write(self.format.as_bytes())?;
        Ok(cursor)
    }
}

impl RiffHeader<'_> {
    fn reflect_double_speed(&mut self) {
        self.size = ((self.size - 36) / 2) + 36;
    }
}

struct FormatSubchunk<'a> {
    id: &'a str,
    size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

impl<'a> ChunkIO<FormatSubchunk<'a>> for FormatSubchunk<'a> {
    fn new(buf: &[u8]) -> FormatSubchunk<'a> {
        if buf.len() != 24 {
            panic!("Format subchunk should be 24 bytes");
        }
        let id = match str::from_utf8(&buf[..4]) {
            Ok("fmt ") => "fmt ",
            _ => panic!("Format subchunk id can only be 'fmt '"),
        };
        let size = read_buffer_as_u32(&buf[4..8]);
        if size != 16 {
            panic!("Format subchunk size should be 16 (PCM)");
        }
        let audio_format = read_buffer_as_u16(&buf[8..10]);
        if audio_format != 1 {
            panic!("Audio format can only be 1 (PCM)");
        }
        let num_channels = read_buffer_as_u16(&buf[10..12]); // 1 = mono, 2 = stereo etc.
        let sample_rate = read_buffer_as_u32(&buf[12..16]); // most likely 44.1k or 48k
        let byte_rate = read_buffer_as_u32(&buf[16..20]); // sample rate * num channels * (bits per sample / 8 (|| "bytes"))
                                                          // TODO: sanity check this
        let block_align = read_buffer_as_u16(&buf[20..22]); // above without sample rate
                                                            // TODO: sanity check this
        let bits_per_sample = read_buffer_as_u16(&buf[22..24]); // normally 8, 32 or in this case, 16
        FormatSubchunk {
            id,
            size,
            audio_format,
            num_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
        }
    }

    fn cursor_over<'b>(
        &self,
        mut cursor: Cursor<Vec<u8>>,
    ) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
        // let mut c = cursor.to_owned();
        cursor.write(self.id.as_bytes())?;
        cursor.write(&self.size.to_le_bytes())?;
        cursor.write(&self.audio_format.to_le_bytes())?;
        cursor.write(&self.num_channels.to_le_bytes())?;
        cursor.write(&self.sample_rate.to_le_bytes())?;
        cursor.write(&self.byte_rate.to_le_bytes())?;
        cursor.write(&self.block_align.to_le_bytes())?;
        cursor.write(&self.bits_per_sample.to_le_bytes())?;
        Ok(cursor)
    }
}

struct DataSubchunk<'a> {
    id: &'a str,
    size: u32,
    data: Vec<u8>,
}

impl<'a> ChunkIO<DataSubchunk<'a>> for DataSubchunk<'a> {
    fn new(buf: &[u8]) -> DataSubchunk<'a> {
        let id = match str::from_utf8(&buf[..4]) {
            Ok("data") => "data",
            _ => panic!("Data subchunk id can only be 'data'"),
        };
        let size = read_buffer_as_u32(&buf[4..8]);
        let mut data: Vec<u8> = Vec::with_capacity(buf.len());
        data.resize(buf.len(), 0);
        data.copy_from_slice(buf);
        DataSubchunk { id, size, data }
    }

    fn cursor_over<'b>(
        &self,
        mut cursor: Cursor<Vec<u8>>,
    ) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
        // let mut c = cursor.to_owned();
        cursor.write(self.id.as_bytes())?;
        cursor.write(&(self.size).to_le_bytes())?;
        cursor.write(&self.data[..])?;
        Ok(cursor)
    }
}

impl DataSubchunk<'_> {
    fn double_speed(&mut self) {
        self.size = self.size / 2;
        self.data = self
            .data
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(i, _)| ((i / 4) % 2) == 0)
            .map(|(_, b)| b)
            .collect::<Vec<u8>>();
    }
}

pub struct Wav<'a> {
    header: RiffHeader<'a>,
    format: FormatSubchunk<'a>,
    data: DataSubchunk<'a>,
}

impl Wav<'_> {
    pub fn new(buf: &[u8]) -> Wav {
        Wav {
            header: RiffHeader::new(&buf[0..12]),
            format: FormatSubchunk::new(&buf[12..36]),
            data: DataSubchunk::new(&buf[36..]),
        }
    }

    pub fn double_speed(&mut self) {
        self.header.reflect_double_speed();
        self.data.double_speed();
        ()
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = Cursor::new(Vec::new());
        writer = self.header.cursor_over(writer)?;
        writer = self.format.cursor_over(writer)?;
        writer = self.data.cursor_over(writer)?;

        let mut file = File::create(filename)?;
        file.write_all(&writer.into_inner())?;

        Ok(())
    }
}

pub fn read_buffer_as_u16(buffer: &[u8]) -> u16 {
    match Cursor::new(buffer).read_u16::<LittleEndian>() {
        Ok(val) => val,
        Err(e) => panic!(e),
    }
}

pub fn read_buffer_as_u32(buffer: &[u8]) -> u32 {
    match Cursor::new(buffer).read_u32::<LittleEndian>() {
        Ok(val) => val,
        Err(e) => panic!(e),
    }
}
