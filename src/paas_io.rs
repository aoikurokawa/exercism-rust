use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    operations_account: usize,
    bytes_account: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            operations_account: 0,
            bytes_account: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_account
    }

    pub fn reads(&self) -> usize {
        self.operations_account
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.operations_account += 1;
        let read = self.wrapped.read(buf)?;
        self.bytes_account += read;
        Ok(read)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    operations_account: usize,
    bytes_account: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            operations_account: 0,
            bytes_account: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_account
    }

    pub fn writes(&self) -> usize {
        self.operations_account
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.operations_account += 1;
        let written = self.wrapped.write(buf)?;
        self.bytes_account += written;
        Ok(written)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}

#[cfg(test)]
mod tests {
    use crate::paas_io as paasio;
    /// test a few read scenarios
    macro_rules! test_read {
    ($(#[$attr:meta])* $modname:ident ($input:expr, $len:expr)) => {
        mod $modname {
            use std::io::{Read, BufReader};
            const CHUNK_SIZE: usize = 2;
            $(#[$attr])*
            #[test]
            fn test_read_passthrough() {
                let data = $input;
                let size = $len(&data);
                let mut reader = crate::ReadStats::new(data);
                let mut buffer = Vec::with_capacity(size);
                let qty_read = reader.read_to_end(&mut buffer);
                assert!(qty_read.is_ok());
                assert_eq!(size, qty_read.unwrap());
                assert_eq!(size, buffer.len());
                // 2: first to read all the data, second to check that
                // there wasn't any more pending data which simply didn't
                // fit into the existing buffer
                assert_eq!(2, reader.reads());
                assert_eq!(size, reader.bytes_through());
            }
            $(#[$attr])*
            #[test]
            fn test_read_chunks() {
                let data = $input;
                let size = $len(&data);
                let mut reader = crate::ReadStats::new(data);
                let mut buffer = [0_u8; CHUNK_SIZE];
                let mut chunks_read = 0;
                while reader.read(&mut buffer[..]).unwrap_or_else(|_| panic!("read failed at chunk {}", chunks_read+1)) > 0 {
                    chunks_read += 1;
                }
                assert_eq!(size / CHUNK_SIZE + std::cmp::min(1, size % CHUNK_SIZE), chunks_read);
                // we read once more than the number of chunks, because the final
                // read returns 0 new bytes
                assert_eq!(1+chunks_read, reader.reads());
                assert_eq!(size, reader.bytes_through());
            }
            $(#[$attr])*
            #[test]
            fn test_read_buffered_chunks() {
                let data = $input;
                let size = $len(&data);
                let mut reader = BufReader::new(crate::ReadStats::new(data));
                let mut buffer = [0_u8; CHUNK_SIZE];
                let mut chunks_read = 0;
                while reader.read(&mut buffer[..]).unwrap_or_else(|_| panic!("read failed at chunk {}", chunks_read+1)) > 0 {
                    chunks_read += 1;
                }
                assert_eq!(size / CHUNK_SIZE + std::cmp::min(1, size % CHUNK_SIZE), chunks_read);
                // the BufReader should smooth out the reads, collecting into
                // a buffer and performing only two read operations:
                // the first collects everything into the buffer,
                // and the second ensures that no data remains
                assert_eq!(2, reader.get_ref().reads());
                assert_eq!(size, reader.get_ref().bytes_through());
            }
        }
    };
}
    /// test a few write scenarios
    macro_rules! test_write {
    ($(#[$attr:meta])* $modname:ident ($input:expr, $len:expr)) => {
        mod $modname {
            use std::io::{self, Write, BufWriter};
            const CHUNK_SIZE: usize = 2;
            $(#[$attr])*
            #[test]
            fn test_write_passthrough() {
                let data = $input;
                let size = $len(&data);
                let mut writer = crate::WriteStats::new(Vec::with_capacity(size));
                let written = writer.write(data);
                assert!(written.is_ok());
                assert_eq!(size, written.unwrap());
                assert_eq!(size, writer.bytes_through());
                assert_eq!(1, writer.writes());
                assert_eq!(data, writer.get_ref().as_slice());
            }
            $(#[$attr])*
            #[test]
            fn test_sink_oneshot() {
                let data = $input;
                let size = $len(&data);
                let mut writer = crate::WriteStats::new(io::sink());
                let written = writer.write(data);
                assert!(written.is_ok());
                assert_eq!(size, written.unwrap());
                assert_eq!(size, writer.bytes_through());
                assert_eq!(1, writer.writes());
            }
            $(#[$attr])*
            #[test]
            fn test_sink_windowed() {
                let data = $input;
                let size = $len(&data);
                let mut writer = crate::WriteStats::new(io::sink());
                let mut chunk_count = 0;
                for chunk in data.chunks(CHUNK_SIZE) {
                    chunk_count += 1;
                    let written = writer.write(chunk);
                    assert!(written.is_ok());
                    assert_eq!(CHUNK_SIZE, written.unwrap());
                }
                assert_eq!(size, writer.bytes_through());
                assert_eq!(chunk_count, writer.writes());
            }
            $(#[$attr])*
            #[test]
            fn test_sink_buffered_windowed() {
                let data = $input;
                let size = $len(&data);
                let mut writer = BufWriter::new(crate::WriteStats::new(io::sink()));
                for chunk in data.chunks(CHUNK_SIZE) {
                    let written = writer.write(chunk);
                    assert!(written.is_ok());
                    assert_eq!(CHUNK_SIZE, written.unwrap());
                }
                // at this point, nothing should have yet been passed through to
                // our writer
                assert_eq!(0, writer.get_ref().bytes_through());
                assert_eq!(0, writer.get_ref().writes());
                // after flushing, everything should pass through in one go
                assert!(writer.flush().is_ok());
                assert_eq!(size, writer.get_ref().bytes_through());
                assert_eq!(1, writer.get_ref().writes());
            }
        }
    };
}
    #[test]
    fn test_create_stats() {
        let mut data: Vec<u8> = Vec::new();
        let _ = paasio::ReadStats::new(data.as_slice());
        let _ = paasio::WriteStats::new(data.as_mut_slice());
    }

    test_read!(#[ignore] read_string (
    "Twas brillig, and the slithy toves/Did gyre and gimble in the wabe:/All mimsy were the borogoves,/And the mome raths outgrabe.".as_bytes(),
    |d: &[u8]| d.len()
));

    test_write!( write_string (
    "Beware the Jabberwock, my son!/The jaws that bite, the claws that catch!/Beware the Jubjub bird, and shun/The frumious Bandersnatch!".as_bytes(),
    |d: &[u8]| d.len()
));

    test_read!(read_byte_literal(
        &[1_u8, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144][..],
        |d: &[u8]| d.len()
    ));

    test_write!(write_byte_literal(
        &[2_u8, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,][..],
        |d: &[u8]| d.len()
    ));

    test_read!(read_file(
        ::std::fs::File::open("Cargo.toml").expect("Cargo.toml must be present"),
        |f: &::std::fs::File| f.metadata().expect("metadata must be present").len() as usize
    ));

    #[test]
    fn read_stats_by_ref_returns_wrapped_reader() {
        use paasio::ReadStats;
        let input =
            "Why, sometimes I've believed as many as six impossible things before breakfast"
                .as_bytes();
        let reader = ReadStats::new(input);
        assert_eq!(reader.get_ref(), &input);
    }
}
