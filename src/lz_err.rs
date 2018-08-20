use std::io;


#[derive(Debug)]
pub enum LzErr {
    NotFound,
    NoParse(String),
    NoFileRead,
    NoItemsFound,
    IOErr(io::Error),
}

impl From<io::Error> for LzErr{
    fn from(e:io::Error)->Self{
        LzErr::IOErr(e)
    }
}
