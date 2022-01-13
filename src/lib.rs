mod ext;
pub mod object_metadata;
mod response_body;
mod uri;

use async_trait::async_trait;

use reqwest::header::HeaderValue;
use response_body::{Item, ResponseBody};
use tokio::io::AsyncRead;

use libunftp::auth::UserDetail;
use libunftp::storage::{Error, ErrorKind, Fileinfo, Metadata, StorageBackend};
use object_metadata::ObjectMetadata;
use reqwest::{header, Body, Client, Url};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};
use tokio::io::BufReader;
use tokio_util::codec::{BytesCodec, FramedRead};
use uri::HttpUri;
pub use ext::ServerExt;

#[derive(Clone, Debug)]
pub struct HttpFileSystem {
    uris: HttpUri,
    client: Client,
    token: String,
}

impl HttpFileSystem {
    pub fn new<Str>(base_url: Str, bucket: Str, token: Str) -> Self
    where
        Str: Into<String>,
    {
        let uris = HttpUri::new(base_url.into(), bucket.into());
        let client = Client::new();
        let token = token.into();
        HttpFileSystem {
            uris,
            client,
            token,
        }
    }
}

#[async_trait]
impl<User: UserDetail> StorageBackend<User> for HttpFileSystem {
    type Metadata = ObjectMetadata;

    async fn metadata<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &User,
        path: P,
    ) -> Result<Self::Metadata, Error> {
        let url: Url = self.uris.metadata(path)?;

        let client: Client = self.client.clone();

        let token = self.token.clone();

        if let Ok(res) = client.post(url)
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
            )
            .send()
            .await {
                if let Ok(item) = res.json::<Item>().await {
                    item.to_metadata()
                } else {
                    Err(Error::from(ErrorKind::PermanentFileNotAvailable))
                }
            } else {
                Err(Error::from(ErrorKind::PermanentFileNotAvailable))
            }
    }

    async fn list<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &User,
        path: P,
    ) -> Result<Vec<Fileinfo<PathBuf, Self::Metadata>>, Error>
    where
        <Self as StorageBackend<User>>::Metadata: Metadata,
    {
        let url: Url = self.uris.list(path)?;

        let client: Client = self.client.clone();

        let token = self.token.clone();

        if let Ok(res) = client.get(url)
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
            )
            .send()
            .await {
                if let Ok(response_body) = res.json::<ResponseBody>().await {
                    response_body.list()
                } else {
                    Err(Error::from(ErrorKind::PermanentFileNotAvailable))
                }
            } else {
                Err(Error::from(ErrorKind::PermanentFileNotAvailable))
            }
    }

    async fn get<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &User,
        _path: P,
        _start_pos: u64,
    ) -> Result<Box<dyn AsyncRead + Send + Sync + Unpin>, Error> {
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn put<P: AsRef<Path> + Send + Debug, B: AsyncRead + Send + Sync + Unpin + 'static>(
        &self,
        _user: &User,
        bytes: B,
        path: P,
        _start_pos: u64,
    ) -> Result<u64, Error> {
        let url: Url = self.uris.put(path)?;

        let client: Client = self.client.clone();

        let reader = BufReader::with_capacity(4096, bytes);

        let token = self.token.clone();

        if let Ok(res) = client
            .post(url)
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
            )
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            )
            .body(Body::wrap_stream(FramedRead::new(
                reader,
                BytesCodec::new(),
            )))
            .send()
            .await
        {
            if let Ok(item) = res.json::<Item>().await {
                Ok(item.to_metadata()?.len())
            } else {
                Err(Error::from(ErrorKind::PermanentFileNotAvailable))
            }
        } else {
            Err(Error::from(ErrorKind::PermanentFileNotAvailable))
        }
    }

    async fn del<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &User,
        _path: P,
    ) -> Result<(), Error> {
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn mkd<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &User,
        _path: P,
    ) -> Result<(), Error> {
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn rename<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &User,
        _from: P,
        _to: P,
    ) -> Result<(), Error> {
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn rmd<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &User,
        _path: P,
    ) -> Result<(), Error> {
        Err(Error::from(ErrorKind::CommandNotImplemented))
    }

    async fn cwd<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &User,
        _path: P,
    ) -> Result<(), Error> {
        Ok(())
    }
}
