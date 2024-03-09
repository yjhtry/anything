use std::{net::Ipv4Addr, path::PathBuf};

use tokio::runtime::Runtime;

pub struct OssServer {
    oss_path: PathBuf,
    host: Ipv4Addr,
    port: u16,
}

impl OssServer {
    pub fn new(oss_path: PathBuf, host: Ipv4Addr, port: u16) -> Self {
        OssServer {
            oss_path,
            host,
            port,
        }
    }

    pub fn start(&self) {
        let oss_path = self.oss_path.clone();
        let host = self.host;
        let port = self.port;

        let server = warp::fs::dir(oss_path);
        let server = warp::serve(server).run((host, port));

        let rt = Runtime::new().unwrap();
        rt.block_on(server);
    }
}
