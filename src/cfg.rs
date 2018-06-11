use lzlist;
use flag;
use brace;
use get::SGetter;
use std::path::PathBuf;

pub struct Cfg{
    list:lzlist::LzList,
    loc:PathBuf,
}

impl Cfg{
    //Will return a Cfg even if there are no items.
   
    pub fn load(loc:&str)->Cfg{
        let rloc = &brace::env_replace(loc)
        match lzlist::LzList::load(rloc) {
            Ok(r)=>Cfg{
                list:r,
                loc:PathBuf::from(rloc),
            },
            Err(_)=>Cfg{
                list:lzlist::LzList::empty(),
                loc:PathBuf::from(&brace::env("$PWD")),
            }
        }
    }

    //load_first
    pub fn load_first(fgname:&str,locs:&[&str])->Cfg{
        let floc = flag::Fg{}.get_s(fgname);
        match floc{
            Some(s)=>return Cfg::load(&s),
            _=>{},
        }

        for l in locs {
            let l2 = &brace::env_replace(l);
            match lzlist::LzList::load(l2){
                Ok(r)=>return Cfg{list:r},
                _=>{},
            }
        }
        Cfg{
            list:lzlist::LzList::empty(),
        }
    }

    pub fn localize(s:&str)->PathBuf{
        let res = path.clone();
        res.push(s);
        res
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


