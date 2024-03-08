use std::{ffi::OsStr, path::PathBuf};

/// Returns the kind of file based on its extension.
///
/// The function takes a `PathBuf` as input and extracts the file extension.
/// It then matches the extension to determine the kind of file.
/// The following mappings are used:
/// - Image extensions: jpg, jpeg, png, gif, webp, bmp, ico, tiff, svg
/// - Video extensions: mp4, webm, mkv, flv, vob, ogv, drc, mng, avi, mov
/// - Audio extensions: mp3, wav, flac, aac, ogg, wma, m4a, m4r, m4p, m4b, m4v, 3gp, 3g2
/// - PDF extension: pdf
/// - Other extensions: all other extensions not mentioned above
///
/// # Arguments
///
/// * `path` - A `PathBuf` representing the path to the file.
///
/// # Returns
///
/// A string slice (`&'static str`) representing the kind of file.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use crate::ext::get_kind_from_path;
///
/// let path = PathBuf::from("path/to/file.jpg");
/// let kind = get_kind_from_path(&path);
/// assert_eq!(kind, "image");
/// ```
pub fn get_kind_from_path(path: &PathBuf) -> &'static str {
    let extension = path
        .extension()
        .map(OsStr::to_str)
        .flatten()
        .unwrap_or_default();

    // match extension all img ext to image, video extension to video, audio extension to audio, pdf to pdf, other to other
    match extension {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "ico" | "tiff" | "svg" => "image",
        "mp4" | "webm" | "mkv" | "flv" | "vob" | "ogv" | "drc" | "mng" | "avi" | "mov" => "video",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" | "m4r" | "m4p" | "m4b" | "m4v"
        | "3gp" | "3g2" => "audio",
        "pdf" => "pdf",
        "json" => "json",
        "toml" => "toml",
        "yaml" => "yaml",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_kind_from_path_should_work() {
        // image tests

        let path = PathBuf::from("path/to/file.jpg");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "image");

        let path = PathBuf::from("path/to/file.jpeg");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "image");

        let path = PathBuf::from("path/to/file.png");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "image");

        // video tests
        let path = PathBuf::from("path/to/file.mp4");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "video");

        let path = PathBuf::from("path/to/file.webm");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "video");

        let path = PathBuf::from("path/to/file.mkv");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "video");

        // audio tests
        let path = PathBuf::from("path/to/file.mp3");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "audio");

        let path = PathBuf::from("path/to/file.wav");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "audio");

        let path = PathBuf::from("path/to/file.flac");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "audio");

        // pdf test
        let path = PathBuf::from("path/to/file.pdf");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "pdf");

        // json test
        let path = PathBuf::from("path/to/file.json");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "json");

        // toml test
        let path = PathBuf::from("path/to/file.toml");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "toml");

        // yaml test
        let path = PathBuf::from("path/to/file.yaml");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "yaml");

        // other tests
        let path = PathBuf::from("path/to/file.txt");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "other");

        let path = PathBuf::from("path/to/file");
        let kind = get_kind_from_path(&path);
        assert_eq!(kind, "other");
    }
}
