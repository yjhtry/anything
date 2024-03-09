use std::path::PathBuf;

use oss_service::OssService;

fn main() {
    let server = OssService::new(
        PathBuf::from("/Users/yjh/anything/oss"),
        "127.0.0.1".parse().unwrap(),
        8888,
    );

    server.start();
}
