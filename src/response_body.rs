// Please refer to: https://github.com/bolcom/libunftp/blob/master/crates/unftp-sbe-gcs/src/response_body.rs

use super::ObjectMetadata;
use chrono::prelude::*;
use libunftp::storage::{Error, ErrorKind, Fileinfo};
use serde::{de, Deserialize};
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Deserialize, Debug)]
pub(crate) struct ResponseBody {
    items: Option<Vec<Item>>,
    prefixes: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Item {
    name: String,
    updated: DateTime<Utc>,

    #[serde(default, deserialize_with = "item_size_deserializer")]
    size: u64,
    #[serde(rename = "md5Hash")]
    md5_hash: String,
}

fn item_size_deserializer<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}

impl ResponseBody {
    pub(crate) fn list(self) -> Result<Vec<Fileinfo<PathBuf, ObjectMetadata>>, Error> {
        let files: Vec<Fileinfo<PathBuf, ObjectMetadata>> = self.items.map_or(Ok(vec![]), move |items: Vec<Item>| {
            items
                .iter()
                .filter(|item: &&Item| !item.name.ends_with('/'))
                .map(move |item: &Item| item.to_file_info())
                .collect()
        })?;
        let dirs: Vec<Fileinfo<PathBuf, ObjectMetadata>> = self.prefixes.map_or(Ok(vec![]), |prefixes: Vec<String>| {
            prefixes
                .iter()
                .filter(|prefix| *prefix != "//")
                .map(|prefix| prefix_to_file_info(prefix))
                .collect()
        })?;
        let result: &mut Vec<Fileinfo<PathBuf, ObjectMetadata>> = &mut vec![];
        result.extend(dirs);
        result.extend(files);
        Ok(result.to_vec())
    }
}

impl Item {
    pub(crate) fn to_metadata(&self) -> Result<ObjectMetadata, Error> {
        Ok(ObjectMetadata {
            size: self.size,
            last_updated: Some(self.updated.into()),
            is_file: !self.name.ends_with('/'),
        })
    }

    pub(crate) fn to_file_info(&self) -> Result<Fileinfo<PathBuf, ObjectMetadata>, Error> {
        let path: PathBuf = PathBuf::from(self.name.clone());
        let metadata: ObjectMetadata = self.to_metadata()?;

        Ok(Fileinfo { path, metadata })
    }

    pub(crate) fn to_md5(&self) -> Result<String, Error> {
        let md5 =
            base64::decode(&self.md5_hash).map_err(|e| Error::new(ErrorKind::LocalError, e))?;
        Ok(md5.iter().map(|b| format!("{:02x}", b)).collect())
    }
}

pub(crate) fn prefix_to_file_info(
    prefix: &str,
) -> Result<Fileinfo<PathBuf, ObjectMetadata>, Error> {
    Ok(Fileinfo {
        path: prefix.into(),
        metadata: ObjectMetadata {
            last_updated: Some(SystemTime::now()),
            is_file: false,
            size: 0,
        },
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use libunftp::storage::Metadata;
    use std::time::SystemTime;

    #[test]
    fn to_metadata() {
        let sys_time = SystemTime::now();
        let date_time = DateTime::from(sys_time);

        let item = Item {
            name: "".into(),
            updated: date_time,
            size: 50,
            md5_hash: "".into(),
        };

        let metadata: ObjectMetadata = item.to_metadata().unwrap();
        assert_eq!(metadata.size, 50);
        assert_eq!(metadata.modified().unwrap(), sys_time);
        assert_eq!(metadata.is_file, true);
    }

    #[test]
    fn to_md5() {
        let sys_time = SystemTime::now();
        let date_time = DateTime::from(sys_time);

        let item = Item {
            name: "".into(),
            updated: date_time,
            size: 50,
            md5_hash: "ZTEwYWRjMzk0OWJhNTlhYmJlNTZlMDU3ZjIwZjg4M2U=".into(),
        };

        let buff = b"e10adc3949ba59abbe56e057f20f883e";
        let md5: String = buff.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(item.to_md5().unwrap(), md5);
    }

    #[test]
    fn to_metadata_parse_error() {
        let response: serde_json::error::Result<Item> =
            serde_json::from_str(r#"{"name":"", "updated":"2020-09-01T12:13:14Z", "size":50}"#);
        assert_eq!(response.err().unwrap().is_data(), true);
    }
}
