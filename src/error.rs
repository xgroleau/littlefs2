//! Error result used in the crate

use crate::ll;
use crate::Result;

/// Definition of errors that might be returned by filesystem functionality.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    /// Error code was >=0, operation was successful.
    Success,
    /// Input / output error occurred.
    Io,
    /// File or filesystem was corrupt.
    Corruption,
    /// No entry found with that name.
    NoSuchEntry,
    /// File or directory already exists.
    EntryAlreadyExisted,
    /// Path name is not a directory.
    PathNotDir,
    /// Path specification is to a directory.
    PathIsDir,
    /// Directory was not empty.
    DirNotEmpty,
    /// Bad file descriptor.
    BadFileDescriptor,
    /// File is too big.
    FileTooBig,
    /// Incorrect value specified to function.
    Invalid,
    /// No space left available for operation.
    NoSpace,
    /// No memory available for completing request.
    NoMemory,
    /// No attribute or data available
    NoAttribute,
    /// Filename too long
    FilenameTooLong,
    /// Unknown error occurred, integer code specified.
    Unknown(i32),
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl From<crate::path::Error> for Error {
    fn from(_error: crate::path::Error) -> Self {
        Error::Io
    }
}

impl From<i32> for Error {
    fn from(error_code: i32) -> Error {
        match error_code {
            n if n >= 0 => Error::Success,
            // negative codes
            ll::lfs_error_LFS_ERR_IO => Error::Io,
            ll::lfs_error_LFS_ERR_CORRUPT => Error::Corruption,
            ll::lfs_error_LFS_ERR_NOENT => Error::NoSuchEntry,
            ll::lfs_error_LFS_ERR_EXIST => Error::EntryAlreadyExisted,
            ll::lfs_error_LFS_ERR_NOTDIR => Error::PathNotDir,
            ll::lfs_error_LFS_ERR_ISDIR => Error::PathIsDir,
            ll::lfs_error_LFS_ERR_NOTEMPTY => Error::DirNotEmpty,
            ll::lfs_error_LFS_ERR_BADF => Error::BadFileDescriptor,
            ll::lfs_error_LFS_ERR_FBIG => Error::FileTooBig,
            ll::lfs_error_LFS_ERR_INVAL => Error::Invalid,
            ll::lfs_error_LFS_ERR_NOSPC => Error::NoSpace,
            ll::lfs_error_LFS_ERR_NOMEM => Error::NoMemory,
            ll::lfs_error_LFS_ERR_NOATTR => Error::NoAttribute,
            ll::lfs_error_LFS_ERR_NAMETOOLONG => Error::FilenameTooLong,
            // positive codes should always indicate success
            _ => Error::Unknown(error_code),
        }
    }
}

pub fn result_from<T>(return_value: T, error_code: ll::lfs_error) -> Result<T> {
    let error: Error = error_code.into();
    match error {
        Error::Success => Ok(return_value),
        _ => Err(error),
    }
}
