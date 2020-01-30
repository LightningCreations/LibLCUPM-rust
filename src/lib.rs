use std::path::Path;

pub mod downloader;

pub trait URI{
    fn scheme(&self) -> &str;
    fn host(&self) -> &str;
    fn path(&self) -> &str;
    fn query(&self) -> &str;
}

impl URI for Path{
    fn scheme(&self) -> &str {
        "file"
    }

    fn host(&self) -> &str {
        ""
    }

    fn path(&self) -> &str {
        self.to_str().unwrap()
    }

    fn query(&self) -> &str {
        ""
    }
}