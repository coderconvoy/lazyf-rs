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

    //load_first
    pub fn load_first(fgname:&str,locs:&[&str])->Cfg{
        let floc = flag::Fg{}.get_s(fgname);
        match floc{
            Some(s)=>return Cfg::load(&brace::env_replace(&s)),
            _=>{},
        }

        for l in locs {
            match lzlist::LzList::load(&brace::env_replace(l)){
                Ok(r)=>return Cfg{list:r},
                _=>{},
            }
        }
        Cfg{
            list:lzlist::LzList::empty(),
        }
    }
}

impl <'a> SGetter<(&'a str,&'a str)> for Cfg{
    fn get_s(&self, fc:(&str,&str))->Option<String>{
        let (fg,ct) = fc; 
        let fres = flag::Fg{}.get_s(fg);
        match fres {
            Some(_)=>return fres,
            _=>{},
        }
        self.list.get_s(ct)
    }
}


