use lzlist::{LzList,Lz};
use std::ops::Deref;
use flag;
use brace;
use get::SGetter;
use std::path::PathBuf;

pub struct Cfg{
    list:LzList,
    loc:PathBuf,
}

impl Cfg{
    //Will return a Cfg even if there are no items.
    pub fn load(loc:&str)->Cfg{
        let rloc = &brace::env_replace(loc);
        match LzList::load(rloc) {
            Ok(r)=>Cfg{
                list:r,
                loc:PathBuf::from(rloc),
            },
            Err(_)=>Cfg{
                list:LzList::empty(),
                loc:PathBuf::from(&brace::env("$PWD")),
            }
        }
    }

    //load_first
    pub fn load_first<'a,L,S>(fgname:&str,locs:L)->Cfg
        where L:IntoIterator<Item=S>,
            S:AsRef<str>,
    {
        let floc = flag::Fg{}.get_s(fgname);
        match floc{
            Some(s)=>return Cfg::load(&s),
            _=>{},
        }

        for l in locs {
            let l2 = &brace::env_replace(&l.as_ref());
            match LzList::load(l2){
                Ok(r)=>return Cfg{list:r,loc:PathBuf::from(l2)},
                _=>{},
            }
        }
        Cfg{
            list:LzList::empty(),
            loc:PathBuf::from(""),
        }
    }

    pub fn localize(&self,s:&str)->PathBuf{
        let mut res = self.folder();
        res.push(s);
        res
    }

    pub fn folder(&self)->PathBuf{
        match self.loc.parent(){
            Some(r)=>PathBuf::from(r),
            _ => PathBuf::new(),
        }
    }
    pub fn lz_by_name(&self,s:&str)->Option<Lz>{
        self.list.lz_by_name(s)
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


