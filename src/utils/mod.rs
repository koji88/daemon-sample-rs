extern crate libc;
extern crate nix;

#[cfg(test)]
extern crate tempdir;

use std::io;
use std::path::Path;
use self::libc::pid_t;
use self::nix::{Error, Errno, sys};

pub fn is_pid_exists(pid: pid_t) -> bool {
    if pid <= 0 {
        return false;
    }

    match sys::signal::kill(pid, sys::signal::SIGHUP) {
        Ok(_) => true,
        Err(Error::Sys(Errno::ESRCH)) => false,
        Err(Error::Sys(Errno::EPERM)) => true,
        _ => false,
    }
}

pub fn write_pid(path: &Path, pid: pid_t) -> io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = try!(File::create(path));
    try!(file.write_fmt(format_args!("{}", pid)));
    Ok(())
}

pub fn read_pid(path: &Path) -> io::Result<pid_t> {
    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;

    let mut file = try!(File::open(path));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    let pid: pid_t = try!(FromStr::from_str(&s)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "cannot parse pid file")));
    Ok(pid)
}


#[cfg(test)]
mod test_utils {
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use utils::tempdir::TempDir;
    use utils;

    #[test]
    fn test_is_pid_exists() {
        assert!(!utils::is_pid_exists(0));
        assert!(utils::is_pid_exists(1));
    }

    #[test]
    fn test_write_pid() {
        let testdir = TestDir::new();
        let path = testdir.dir.path().join("test.pid");
        assert!(utils::write_pid(&path, 100).is_ok());
        let r = utils::read_pid(&path);
        assert!(r.is_ok());
        assert!(r.unwrap() == 100);

    }

    struct TestDir {
        dir: TempDir,
    }

    impl TestDir {
        fn new() -> TestDir {
            TestDir { dir: TempDir::new("daemon-sample-rs").unwrap() }
        }

        fn new_file(&self, filename: &str) -> (PathBuf, PathBuf, File) {
            let path = self.dir.path().join(filename);
            let file = File::create(&path)
                .unwrap_or_else(|error| panic!("Failed to create temporary file: {}", error));

            (self.dir.path().to_path_buf(), path, file)
        }
    }

    fn write_to(file: &mut File) {
        file.write(b"This should trigger an inotify event.")
            .unwrap_or_else(|error| panic!("Failed to write to file: {}", error));
    }

}
