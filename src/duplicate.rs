use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};
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

#[derive(Debug)]
pub struct Fragment {
    pub path: String,
    pub content: String,
    pub start_offset: u64,
    pub end_offset: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    #[test]
    fn tt() -> Result<()> {
        let path = "foo.bar";
        let mut file = File::create(path)?;
        let mut d = Duplicator::new(path)?;

        for _ in 0..5 {
            for j in 0..10 {
                file.write_all(format!("{}\n", j).as_bytes())?;
            }
            assert_eq!("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n", d.duplicate()?.content);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
        std::fs::remove_file(path).unwrap();
        Ok(())
    }
}
