use unicode_segmentation::UnicodeSegmentation;

pub mod admin;
pub mod common;
pub mod db;
pub mod request;
pub mod response;

// API compatibility
trait IntToBool {
    fn to_bool(self) -> bool;
}
trait BoolToInt {
    fn to_i32(self) -> i32;
}

impl IntToBool for i32 {
    fn to_bool(self) -> bool {
        self > 0
    }
}
impl BoolToInt for bool {
    fn to_i32(self) -> i32 {
        self as i32
    }
}

trait TruncatedString {
    fn truncated(&self, len: usize) -> String;
}

impl TruncatedString for String {
    fn truncated(&self, len: usize) -> String {
        self.graphemes(true).take(len).collect()
    }
}
