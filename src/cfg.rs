use lzlist;
use flag;

pub struct Cfg{
    list:lzlist::LzList,
}

impl Cfg{
    //Will return a Cfg even if there are no items.
    pub fn load(fgname:&str,locs:&Iterator<Item=&str>)->Cfg{
        Cfg{
            list:lzlist::LzList::empty(),
        }
    }
}
