#[tokio::main]
async fn main() {
    tokio::fs::copy("src/lib.rs", "examples/b.rs")
        .await
        .unwrap();
}
