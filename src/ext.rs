use crate::HttpFileSystem;
use libunftp::auth::DefaultUser;
use libunftp::Server;

pub trait ServerExt {
    fn with_http<Str>(base_url: Str, bucket: Str, token: Str) -> Server<HttpFileSystem, DefaultUser> 
    where
        Str: Into<String>,
        {
            let url = base_url.into();
            let b = bucket.into();
            let t = token.into();
            Server::new(Box::new(move || HttpFileSystem::new(url.clone(), b.clone(), t.clone())))
        }
}

impl ServerExt for Server<HttpFileSystem, DefaultUser> {}