use std::fs::File;
use std::io::{
    Write,
    Result,
};
use tempdir;

enum MaybeRec {
    SomeVariant(i64),
    OtherVariant(String),
    RecVariant(Box<MaybeRec>),
}

fn main() {
    println!("Hello, world!");

    let x = MaybeRec::SomeVariant(12);
    let y = MaybeRec::OtherVariant(String::from("hi"));
    let z = MaybeRec::RecVariant(Box::new(x));
    let aa = MaybeRec::RecVariant(Box::new(
        MaybeRec::RecVariant(Box::new(
            MaybeRec::RecVariant(Box::new(y))
            )
        )
    ));
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// type IoResult<T> = Result<T, std::io::Error>;

#[derive(Debug)]
struct DurableFile {
    file: File,
    needs_sync: bool,
}

#[derive(Debug)]
struct CloseError {
    file: DurableFile,
    error: std::io::Error,
}

impl Write for DurableFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.file.write(buf) {
            Ok(amt) => {
                self.needs_sync = true;
                Ok(amt)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    // Result<bool>  // bool is the type
    // Ok(true)      // true is the value (of type bool)

    // Result<()>  // () as a type
    // Ok(())      // () as a value (of type ())

    fn flush(&mut self) -> Result<()> {
        match self.file.sync_all() {
            Ok(()) => {
                self.needs_sync = false;
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
}

impl Drop for DurableFile {
    fn drop(&mut self) {
        // Any edge cases?
        if self.needs_sync {
            panic!("You forgot to sync!");
        }
    }
}

impl DurableFile {
    pub fn new(file: File) -> DurableFile {
        DurableFile {
            file,
            needs_sync: false,
        }
    }

    pub fn close(mut self) -> std::result::Result<(), CloseError> {
        match self.flush() {
            Ok(()) => {
                Ok(())
            }
            Err(e) => {
                Err(CloseError { file: self, error: e })
            }
        }
    }
}



#[test]
fn create_no_write() {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file = std::fs::File::create(dir.path().join("foo.txt")).unwrap();
    let _durable = DurableFile::new(file);

    // No writes, let the file drop naturally.
}

#[test]
#[should_panic(expected = "You forgot to sync!")]
fn create_write_panics() {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file = std::fs::File::create(dir.path().join("foo.txt")).unwrap();
    let mut durable = DurableFile::new(file);
    durable.write_all(b"Hello, world!").unwrap();
    // Should panic, we forgot to sync!
}

#[test]
fn create_write_sync() {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file = std::fs::File::create(dir.path().join("foo.txt")).unwrap();
    let mut durable = DurableFile::new(file);
    durable.write_all(b"Hello, world!").unwrap();
    durable.flush().unwrap();

    // We now drop, shouldn't panic because we flush'd.
}

#[test]
fn create_write_close() {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file = std::fs::File::create(dir.path().join("foo.txt")).unwrap();
    let mut durable = DurableFile::new(file);
    durable.write_all(b"Hello, world!").unwrap();

    // This will close and drop the durable file, it
    // shouldn't panic because we closed it manually.
    durable.close().unwrap();
}
