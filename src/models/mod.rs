pub mod common;
pub mod db;
pub mod request;
pub mod response;

// API compatibility
trait IntToBool {
    fn to_bool(self) -> bool;
}

impl IntToBool for i32 {
    fn to_bool(self) -> bool {
        self > 0
    }
}

// Simplification, since String::truncate modifies string inplace
trait TruncatedString {
    fn truncated(&self, len: usize) -> String;
}

impl TruncatedString for String {
    fn truncated(&self, len: usize) -> String {
        let mut s = self.clone();
        s.truncate(len);
        s
    }
}
