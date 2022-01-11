mod uri;
mod response_body;
pub mod object_metadata;

use async_trait::async_trait;

use libunftp::storage::{Error, ErrorKind, Fileinfo, Metadata, StorageBackend};
use reqwest::Client;
use uri::HttpUri;
use object_metadata::ObjectMetadata;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

pub struct HttpFileSystem {
    uri: HttpUri,
}

impl HttpFileSystem {
    // pub fn new<Str>(base_url: Str, bucket: Str) -> Self where Str: Into<String> {

    // }
}

#[async_trait]
impl<U: Sync + Send + Debug> StorageBackend<U> for HttpFileSystem {
    type Metadata = ObjectMetadata;

    fn supported_features(&self) -> u32 {
        libunftp::storage::FEATURE_SITEMD5
    }

    async fn metadata<P: AsRef<Path> + Send + Debug>(&self, _user: &Option<U>, path: P) -> Result<Self::Metadata, Error> {
        // TODO: implement this
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn list<P: AsRef<Path> + Send + Debug>(&self, _user: &Option<U>, path: P) -> Result<Vec<Fileinfo<PathBuf, Self::Metadata>>, Error>
    where
        <Self as StorageBackend<U>>::Metadata: Metadata,
    {
        // TODO: implement this
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn get<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &Option<U>,
        path: P,
        _start_pos: u64,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Send + Sync + Unpin>, Error> {
        // TODO: implement this
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn put<P: AsRef<Path> + Send + Debug, B: tokio::io::AsyncRead + Send + Sync + Unpin + 'static>(
        &self,
        _user: &Option<U>,
        bytes: B,
        path: P,
        _start_pos: u64,
    ) -> Result<u64, Error> {
        // TODO: implement this
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn del<P: AsRef<Path> + Send + Debug>(&self, _user: &Option<U>, path: P) -> Result<(), Error> {
        // TODO: implement this
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn mkd<P: AsRef<Path> + Send + Debug>(&self, _user: &Option<U>, path: P) -> Result<(), Error> {
        // TODO: implement this
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn rename<P: AsRef<Path> + Send + Debug>(&self, _user: &Option<U>, _from: P, _to: P) -> Result<(), Error> {
        // TODO: implement this
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn rmd<P: AsRef<Path> + Send + Debug>(&self, _user: &Option<U>, _path: P) -> Result<(), Error> {
        // TODO: implement this
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn cwd<P: AsRef<Path> + Send + Debug>(&self, _user: &Option<U>, _path: P) -> Result<(), Error> {
        // TODO: Do we want to check here if the path is a directory?
        Ok(())
    }
}