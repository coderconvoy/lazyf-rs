use lzlist;
use flag;
use brace;
use get::SGetter;

pub struct Cfg{
    list:lzlist::LzList,
}

impl Cfg{
    //Will return a Cfg even if there are no items.
   
    pub fn load(loc:&str)->Cfg{
        match lzlist::LzList::load(loc) {
            Ok(r)=>Cfg{
                list:r,
            },
            Err(_)=>Cfg{
                list:lzlist::LzList::empty(),
            }
        }
    }

    pub fn load_first(fgname:&str,locs:&Iterator<Item=&str>)->Cfg{
        let floc = flag::Fg{}.get_s(fgname);
        match floc{
            Some(s)=>return Cfg::load(&brace::brace_env(&s)),
            _=>{},
        }

        //TODO loop loads and match to get first working
        Cfg{
            list:lzlist::LzList::empty(),
        }
    }
}
