use crate::ffi::{OsStr, OsString};
use crate::marker::PhantomData;
use crate::path::{self, Path, PathBuf};
use crate::sys::pal::unsupported;
use crate::{fmt, io};

pub fn getcwd() -> io::Result<PathBuf> {
    unsupported()
}

pub fn chdir(_: &path::Path) -> io::Result<()> {
    unsupported()
}

pub struct SplitPaths<'a>(crate::vec::IntoIter<PathBuf>, PhantomData<&'a ()>);
pub struct SplitPathsRef<'a>(core::str::Split<'a, char>);

pub fn split_paths(unparsed: &OsStr) -> SplitPaths<'_> {
    let text = unparsed.to_string_lossy();
    SplitPaths(text.split(':').map(PathBuf::from).collect::<Vec<_>>().into_iter(), PhantomData)
}

pub fn split_paths_ref(unparsed: &OsStr) -> Option<SplitPathsRef<'_>> {
    Some(SplitPathsRef(unparsed.to_str()?.split(':')))
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0.next()
    }
}

impl<'a> Iterator for SplitPathsRef<'a> {
    type Item = &'a Path;
    fn next(&mut self) -> Option<&'a Path> {
        self.0.next().map(Path::new)
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    let mut out = OsString::new();
    for path in paths {
        if !out.is_empty() {
            out.push(":");
        }
        out.push(path);
    }
    Ok(out)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on this platform yet".fmt(f)
    }
}

impl crate::error::Error for JoinPathsError {}

pub fn current_exe() -> io::Result<PathBuf> {
    Err(io::Error::from(io::ErrorKind::Unsupported))
}

pub fn temp_dir() -> PathBuf {
    panic!("no filesystem on this platform")
}

pub fn home_dir() -> Option<PathBuf> {
    crate::env::var_os("HOME").map(PathBuf::from)
}
