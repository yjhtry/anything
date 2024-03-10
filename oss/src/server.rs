use std::{net::Ipv4Addr, path::PathBuf, thread::spawn};

use tokio::runtime::Runtime;
use warp::Filter;

pub struct OssService {
    pub oss_path: PathBuf,
    pub host: Ipv4Addr,
    pub port: u16,
}

impl OssService {
    pub fn new(oss_path: PathBuf, host: Ipv4Addr, port: u16) -> Self {
        OssService {
            oss_path,
            host,
            port,
        }
    }

    pub fn start(&self) {
        let oss_path = self.oss_path.clone();
        let host = self.host;
        let port = self.port;

        let cors = warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST", "DELETE"]);

        let server = warp::fs::dir(oss_path).with(cors);
        let server = warp::serve(server).run((host, port));

        spawn(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(server);
        });
    }
}
