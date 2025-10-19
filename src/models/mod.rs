use unicode_segmentation::UnicodeSegmentation;

pub mod admin;
pub mod common;
pub mod db;
pub mod request;
pub mod response;

trait TruncatedString {
    fn truncated(&self, len: usize) -> String;
}

impl TruncatedString for String {
    fn truncated(&self, len: usize) -> String {
        self.graphemes(true).take(len).collect()
    }
}
