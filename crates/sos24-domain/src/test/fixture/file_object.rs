use crate::entity::file_object::{FileObject, FileObjectKey, FileSignedUrl};

pub fn data() -> Vec<u8> {
    b"hello world".to_vec()
}

pub fn key() -> FileObjectKey {
    FileObjectKey::new("key1".to_string())
}

pub fn file_object() -> FileObject {
    FileObject::new(data(), key())
}

pub fn signed_url() -> FileSignedUrl {
    FileSignedUrl::new(url::Url::parse("https://example.com").unwrap())
}
