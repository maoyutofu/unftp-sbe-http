// Please refer to: https://github.com/bolcom/libunftp/blob/master/crates/unftp-sbe-gcs/src/object_metadata.rs

use libunftp::storage::Metadata;
use libunftp::storage::{Error, ErrorKind};
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct ObjectMetadata {
    pub(crate) last_updated: Option<SystemTime>,
    pub(crate) is_file: bool,
    pub(crate) size: u64,
}

impl Metadata for ObjectMetadata {
    fn len(&self) -> u64 {
        self.size
    }

    fn is_dir(&self) -> bool {
        !self.is_file()
    }

    fn is_file(&self) -> bool {
        self.is_file
    }

    fn modified(&self) -> Result<SystemTime, Error> {
        match self.last_updated {
            Some(timestamp) => Ok(timestamp),
            None => Err(Error::from(ErrorKind::PermanentFileNotAvailable)),
        }
    }

    fn is_symlink(&self) -> bool {
        // TODO: implement this
        false
    }

    fn gid(&self) -> u32 {
        // TODO: implement this
        0
    }

    fn uid(&self) -> u32 {
        // TODO: implement this
        0
    }
}
