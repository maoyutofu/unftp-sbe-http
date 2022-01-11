mod uri;
mod response_body;
pub mod object_metadata;

use async_trait::async_trait;

use reqwest::header::HeaderValue;
use tokio::io::AsyncRead;

use libunftp::auth::UserDetail;
use libunftp::storage::{Error, Fileinfo, Metadata, StorageBackend};
use tokio_util::codec::{BytesCodec, FramedRead};
use uri::HttpUri;
use object_metadata::ObjectMetadata;
use reqwest::{Body, Client, Url, header};
use tokio::io::BufReader;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct HttpFileSystem {
    uris: HttpUri,
    client: Client,
    token: String,
}

impl HttpFileSystem {
    pub fn new<Str>(base_url: Str, bucket: Str, root: PathBuf, token: Str) -> Self where Str: Into<String> {
        let uris = HttpUri::new(base_url.into(), bucket.into(), root);
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

    async fn metadata<P: AsRef<Path> + Send + Debug>(&self, _user: &User, _path: P) -> Result<Self::Metadata, Error> {
        todo!()
    }

    async fn list<P: AsRef<Path> + Send + Debug>(&self, _user: &User, _path: P) -> Result<Vec<Fileinfo<PathBuf, Self::Metadata>>, Error>
    where
        <Self as StorageBackend<User>>::Metadata: Metadata,
    {
        todo!()
    }

    async fn get<P: AsRef<Path> + Send + Debug>(&self, _user: &User, _path: P, _start_pos: u64) -> Result<Box<dyn AsyncRead + Send + Sync + Unpin>, Error> {
        todo!()
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

        let res = client.post(url)
            .header(header::AUTHORIZATION, HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap())
            .header(header::CONTENT_TYPE, HeaderValue::from_static("application/octet-stream"))
            .body(Body::wrap_stream(FramedRead::new(reader, BytesCodec::new())))
            .send().await;
            todo!()
    }

    async fn del<P: AsRef<Path> + Send + Debug>(&self, _user: &User, _path: P) -> Result<(), Error> {
        todo!()
    }

    async fn mkd<P: AsRef<Path> + Send + Debug>(&self, _user: &User, _path: P) -> Result<(), Error> {
        todo!()
    }

    async fn rename<P: AsRef<Path> + Send + Debug>(&self, _user: &User, _from: P, _to: P) -> Result<(), Error> {
        todo!()
    }

    async fn rmd<P: AsRef<Path> + Send + Debug>(&self, _user: &User, _path: P) -> Result<(), Error> {
        todo!()
    }

    async fn cwd<P: AsRef<Path> + Send + Debug>(&self, _user: &User, _path: P) -> Result<(), Error> {
        todo!()
    }
}