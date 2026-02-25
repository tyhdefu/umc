use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, stderr, stdin, stdout};

use int_enum::IntEnum;

pub type FileHandle = u32;

#[derive(IntEnum)]
#[repr(u32)]
pub enum ECallCode {
    EXIT = 0x0,
    OPEN = 0x1,
    CLOSE = 0x2,
    READ = 0x3,
    WRITE = 0x4,
}

/// The given file name could not be opened
#[derive(Debug)]
pub struct OpenFileError<'a> {
    filename: &'a str,
}

#[derive(Debug)]
pub struct InvalidFileHandle(FileHandle);

/// Platform specific interactions that implement the environment
pub trait Environment {
    fn open<'a, 'b>(&'a mut self, filename: &'b str) -> Result<FileHandle, OpenFileError<'b>>;

    fn close(&mut self, handle: FileHandle) -> Result<(), InvalidFileHandle>;

    fn read(&mut self, handle: FileHandle, buf: &mut [u8]) -> Result<usize, InvalidFileHandle>;

    fn write(&mut self, handle: FileHandle, buf: &[u8]) -> Result<usize, InvalidFileHandle>;
}

impl Environment for AnyEnvironment {
    fn open<'a, 'b>(&'a mut self, filename: &'b str) -> Result<FileHandle, OpenFileError<'b>> {
        let file: File = File::options()
            .read(true)
            .write(true)
            .open(filename)
            .map_err(|_| OpenFileError { filename })?;
        let handle = self.next_file_id;
        self.open_files.insert(handle, OpenFile::File(file));
        self.next_file_id += 1;
        Ok(handle)
    }

    fn close(&mut self, handle: FileHandle) -> Result<(), InvalidFileHandle> {
        match self.open_files.remove(&handle) {
            Some(_) => Ok(()),
            None => Err(InvalidFileHandle(handle)),
        }
    }

    fn read(&mut self, handle: FileHandle, buf: &mut [u8]) -> Result<usize, InvalidFileHandle> {
        let open_file = self
            .open_files
            .get_mut(&handle)
            .ok_or(InvalidFileHandle(handle))?;

        Ok(open_file.read(buf).unwrap_or(0))
    }

    fn write(&mut self, handle: FileHandle, buf: &[u8]) -> Result<usize, InvalidFileHandle> {
        let open_file = self
            .open_files
            .get_mut(&handle)
            .ok_or(InvalidFileHandle(handle))?;

        Ok(open_file.write(buf).unwrap_or(0))
    }
}

/// An environment implementation that delegates to the Rust standard library
pub struct AnyEnvironment {
    open_files: HashMap<FileHandle, OpenFile>,
    next_file_id: u32,
}

impl AnyEnvironment {
    pub fn new() -> Self {
        let mut open_files = HashMap::new();
        open_files.insert(0, OpenFile::StdIn);
        open_files.insert(1, OpenFile::StdOut);
        open_files.insert(2, OpenFile::StdErr);

        Self {
            open_files: open_files,
            next_file_id: 3,
        }
    }
}

enum OpenFile {
    StdIn,
    StdOut,
    StdErr,
    File(File),
}

impl OpenFile {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        match self {
            OpenFile::StdIn => stdin().read(buf).map_err(|_| ()),
            OpenFile::StdOut => Err(()), // Can you read from stdout?
            OpenFile::StdErr => Err(()),
            OpenFile::File(file) => file.read(buf).map_err(|_| ()),
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        match self {
            OpenFile::StdIn => Err(()),
            OpenFile::StdOut => stdout().write(buf).map_err(|_| ()),
            OpenFile::StdErr => stderr().write(buf).map_err(|_| ()),
            OpenFile::File(file) => file.write(buf).map_err(|_| ()),
        }
    }
}
