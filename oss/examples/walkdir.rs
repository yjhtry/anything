use std::path::Path;

use oss::is_hidden;
use walkdir::WalkDir;

fn main() {
    let dir = Path::new("/Users/yjh/anything");

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry.unwrap();
        let path = entry.path();

        println!(
            "{}, {}, is_dir: {}, is_file: {}",
            path.file_name().unwrap().to_str().unwrap(),
            path.is_dir(),
            path.is_file(),
            path.display()
        );
    }
}
