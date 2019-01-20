use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

pub struct Duplicator {
    pub path: String,
    pub file: File,
    pub offset: u64,
}

impl Duplicator {
    pub fn new(path: &str) -> Result<Duplicator> {
        Ok(Duplicator {
            path: path.to_owned(),
            file: File::open(path)?,
            offset: 0,
        })
    }

    pub fn duplicate(&mut self) -> Result<Fragment> {
        self.file.seek(SeekFrom::Start(self.offset))?;
        let file_len = self.file.metadata()?.len();
        let mut buf = String::with_capacity((file_len - self.offset) as usize);
        self.file.read_to_string(&mut buf)?;

        let old_offset = self.offset;
        self.offset = file_len;
        Ok(Fragment {
            path: self.path.clone(),
            content: buf,
            start_offset: old_offset,
            end_offset: self.offset,
        })
    }
}

pub struct Fragment {
    pub path: String,
    pub content: String,
    pub start_offset: u64,
    pub end_offset: u64,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
    #[test]
    fn tt() -> Result<()> {
        let mut file = File::create("foo.txt")?;
        file.write_all(b"Hello, world!")?;
        unimplemented!()
    }
}
