use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

pub struct Duplicator {
    path: String,
    file: File,
    offset: u64,
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
    path: String,
    content: String,
    start_offset: u64,
    end_offset: u64,
}
