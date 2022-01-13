// Please refer to: https://github.com/bolcom/libunftp/blob/master/crates/unftp-sbe-gcs/src/uri.rs

use libunftp::storage::{Error, ErrorKind};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Url;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct HttpUri {
    base_url: String,
    bucket: String,
    root: PathBuf,
}

impl HttpUri {
    pub fn new(base_url: String, bucket: String) -> Self {
        let root = PathBuf::new();
        Self {
            base_url,
            bucket,
            root
        }
    }

    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Url, Error> {
        make_uri(format!(
            "{}/v1/b/{}/o/{}",
            self.base_url,
            self.bucket,
            self.path_str(path)?
        ))
    }

    pub fn list<P: AsRef<Path>>(&self, path: P) -> Result<Url, Error> {
        let mut prefix = self.path_str(path)?;
        if !prefix.is_empty() && !prefix.ends_with("%2F") {
            prefix.push_str("%2F");
        }
        make_uri(format!(
            "{}/v1/b/{}/o?prefix={}",
            self.base_url, self.bucket, prefix
        ))
    }

    pub fn get<P: AsRef<Path>>(&self, path: P) -> Result<Url, Error> {
        make_uri(format!(
            "{}/v1/b/{}/o/{}?alt=media",
            self.base_url,
            self.bucket,
            self.path_str(path)?
        ))
    }

    pub fn put<P: AsRef<Path>>(&self, path: P) -> Result<Url, Error> {
        let path = self.path_str(path)?;
        let path = path.trim_end_matches("%2F");

        make_uri(format!(
            "{}/v1/b/{}/o?name={}",
            self.base_url, self.bucket, path
        ))
    }

    pub fn delete<P: AsRef<Path>>(&self, path: P) -> Result<Url, Error> {
        make_uri(format!(
            "{}/v1/b/{}/o/{}",
            self.base_url,
            self.bucket,
            self.path_str(path)?
        ))
    }

    pub fn mkd<P: AsRef<Path>>(&self, path: P) -> Result<Url, Error> {
        let path = self.path_str(path)?;
        let path = path.trim_end_matches("%2F");

        make_uri(format!(
            "{}/v1/b/{}/o?name={}/",
            self.base_url, self.bucket, path
        ))
    }

    fn path_str<P: AsRef<Path>>(&self, path: P) -> Result<String, Error> {
        let path = path.as_ref();
        let relative_path = path.strip_prefix("/").unwrap_or(path);
        if let Some(path) = self.root.join(relative_path).to_str() {
            let result_path = utf8_percent_encode(path, NON_ALPHANUMERIC).collect::<String>();
            Ok(result_path)
        } else {
            Err(Error::from(ErrorKind::PermanentFileNotAvailable))
        }
    }
}

fn make_uri(path_and_query: String) -> Result<Url, Error> {
    Url::parse(&path_and_query).map_err(|_| Error::from(ErrorKind::FileNameNotAllowedError))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list() {
        struct Test {
            sub: &'static str,
            expected_prefix: &'static str,
        }
        let tests = [
            Test {
                sub: "/",
                expected_prefix: "the%2Droot%2F",
            },
            Test {
                sub: "",
                expected_prefix: "the%2Droot%2F",
            },
            Test {
                sub: "/",
                expected_prefix: "the%2Droot%2F",
            },
            Test {
                sub: "",
                expected_prefix: "the%2Droot%2F",
            },
            Test {
                sub: "/the-sub-folder",
                expected_prefix: "the%2Droot%2Fthe%2Dsub%2Dfolder%2F",
            },
            Test {
                sub: "the-sub-folder",
                expected_prefix: "the%2Droot%2Fthe%2Dsub%2Dfolder%2F",
            },
            Test {
                sub: "the-sub-folder",
                expected_prefix: "the%2Droot%2Fthe%2Dsub%2Dfolder%2F",
            },
            Test {
                sub: "/the-sub-folder",
                expected_prefix: "the%2Droot%2Fthe%2Dsub%2Dfolder%2F",
            },
            Test {
                sub: "the-sub-folder/",
                expected_prefix: "the%2Droot%2Fthe%2Dsub%2Dfolder%2F",
            },
            Test {
                sub: "",
                expected_prefix: "",
            },
        ];

        let s = "http://localhost:8088/v1/b/anfang/o?prefix";
        for test in tests.iter() {
            let uri = HttpUri::new(
                "http://localhost:8088".to_string(),
                "anfang".to_string(),
            );
            assert_eq!(
                format!("{}={}", s, test.expected_prefix),
                uri.list(test.sub).unwrap().to_string()
            );
        }
    }
}
