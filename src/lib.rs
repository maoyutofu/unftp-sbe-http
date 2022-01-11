mod uri;
mod response_body;
pub mod object_metadata;

use libunftp::storage::{Error, ErrorKind, Fileinfo, Metadata, StorageBackend};
use reqwest::Client;
use uri::HttpUri;
use object_metadata::ObjectMetadata;
pub struct HttpFileSystem {
    uri: HttpUri,
}

impl HttpFileSystem {
    // pub fn new<Str>(base_url: Str, bucket: Str) -> Self where Str: Into<String> {

    // }
}