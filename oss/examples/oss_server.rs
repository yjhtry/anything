use std::path::PathBuf;

use oss::OssServer;

fn main() {
    let server = OssServer::new(
        PathBuf::from("/Users/yjh/anything/oss"),
        "127.0.0.1".parse().unwrap(),
        8888,
    );

    server.start();
}
