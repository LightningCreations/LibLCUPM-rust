
#[macro_use]
extern crate lazy_static;



use std::io::Read;

pub trait Downloader : Read{}

pub trait DownloaderScheme{
    fn matches<URI: crate::URI>(&self,uri: &URI) -> bool;
    fn download<URI: crate::URI>(&self,uri: URI) -> Result<Box<dyn Downloader>,std::string::String>;
    fn get_name(&self) -> &str;
}

lazy_static!{
    static ref DOWNLOADERS: std::sync::RwLock<std::vec::Vec<&'static dyn DownloaderScheme>> = {
        std::sync::RwLock::new(std::vec::Vec::new())
    };
}



pub fn register<D: DownloaderScheme>(dl: &'static D) -> Result<(),std::string::String>{
    DOWNLOADERS.write()?.push(dl);
    Ok(())
}

pub fn download<URI: crate::URI>(uri: URI) -> Result<Box<dyn Downloader>,std::string::String>{
    for dl in *(DOWNLOADERS.read()?){
        if dl.matches(&uri){
            return dl.download(uri);
        }
    }
    Err(format!("No matching downloader for {} was found",uri))
}
