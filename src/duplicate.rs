use memmap::{MmapOptions};
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

    pub fn duplicate(&mut self) -> Result<DuplicateResult> {
        let file_len = self.file.metadata()?.len();
        let content_len = file_len - self.offset;
        if content_len <= MMAP_THRESHOLD {
            self.file_dup(file_len)
        } else {
            self.mmap_dup(file_len, content_len)
        }
    }

    fn mmap_dup(&mut self, file_len: u64, content_len: u64) -> Result<DuplicateResult> {
        let (state, content_len) = if content_len < FRAGMENT_LENGTH_MAXIMUM {
            (DuplicateState::Done, content_len)
        } else {
            (DuplicateState::OnGoing, FRAGMENT_LENGTH_MAXIMUM)
        };

        let mut mmap_option = MmapOptions::new();
        mmap_option.offset(self.offset).len(content_len as usize);
        let mmap = unsafe { mmap_option.map(&self.file) }?;
        let buf = String::from_utf8_lossy(mmap.as_ref()).into_owned();

        let old_offset = self.offset;
        self.offset = old_offset + FRAGMENT_LENGTH_MAXIMUM;
        Ok(DuplicateResult::new(
            state,
            Fragment {
                path: self.path.clone(),
                content: buf,
                start_offset: old_offset,
                end_offset: self.offset,
            },
        ))
    }

    // in case the length < 4kb
    fn file_dup(&mut self, file_len: u64) -> Result<DuplicateResult> {
        self.file.seek(SeekFrom::Start(self.offset))?;
        let mut buf = String::with_capacity((file_len - self.offset) as usize);
        self.file.read_to_string(&mut buf)?;

        let old_offset = self.offset;
        self.offset = file_len;
        Ok(DuplicateResult::new(
            DuplicateState::Done,
            Fragment {
                path: self.path.clone(),
                content: buf,
                start_offset: old_offset,
                end_offset: self.offset,
            },
        ))
    }
}

pub struct DuplicateResult {
    state: DuplicateState,
    fragment: Fragment,
}

impl DuplicateResult {
    fn new(state: DuplicateState, fragment: Fragment) -> Self {
        DuplicateResult { state, fragment }
    }
}

pub enum DuplicateState {
    OnGoing,
    Done,
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub path: String,
    pub content: String,
    pub start_offset: u64,
    pub end_offset: u64,
}

const MMAP_THRESHOLD: u64 = 4096; // 4kb, the linux page size
const FRAGMENT_LENGTH_MAXIMUM: u64 = 100 * 0x100000; // 100mb, the default max content of an HTTP request in elastic

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn base_file_dup() -> Result<()> {
        let path = "foo.bar";
        let mut file = File::create(path)?;
        let mut d = Duplicator::new(path)?;

        for _ in 0..5 {
            for j in 0..10 {
                file.write_all(format!("{}\n", j).as_bytes())?;
            }
            assert_eq!("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n", d.duplicate()?.fragment.content);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
        std::fs::remove_file(path).unwrap();
        Ok(())
    }
}
