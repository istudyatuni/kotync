use serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct DBStats {
    pub users_count: u32,
    pub manga_count: u64,
}
