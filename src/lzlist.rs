use std::fs::File;
use std::fmt::Debug;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;
//use std::fmt::Display;
use brace;
use get::SGetter;
use lz_err::LzErr;


#[derive(Clone)]
pub struct Lz{
    pub name:String,
    deets:HashMap<String,String>,
}

impl Lz{
    pub fn new(line:&str)->Lz{
        let ss = line.split(":");
        let mut res = Lz{
            name:"".to_string(),
            deets:HashMap::new(),
        };
        for (k,v) in ss.enumerate(){
            if k ==0 {
                res.name = v.trim().to_string();
                continue;
            }
            res.add_deet(&format!("ext{}",k-1),v.trim());
        }
        res
    }

    pub fn add_deet(&mut self,k:&str,v:&str){
        self.deets.insert(k.to_string(),v.to_string());
    }

    
}

impl <'a> SGetter<&'a str> for Lz {
    fn get_s(&self,s :&str)->Option<String>{
        match self.deets.get(s){
            Some(r)=>Some(r.to_string()),
            None=>None,
        }
    }
}


pub struct LzList {
   items:Vec<Lz>,
}

impl LzList {
    pub fn read(s :&String)->Result<LzList,LzErr>{
        let sp = s.split("\n");
        let mut res = LzList{
            items:Vec::new(),
        };
        let mut fnd = false;

        let mut curr = Lz::new("");

        for a in sp {//TODO errors with line nums
            if a == "" {continue;}
            let tabbed = match a.chars().nth(0) {
                Some(c)=>c.is_whitespace(),
                _=> true,
            };
            let a = a.trim_left();

            if a.len() == 0 {continue;}

            if &a[..1] == "#" {continue;}

            if tabbed {
                //new property
                let (lt,rt) = brace::split_on(a,':');
                if rt == "" {
                    //TODO Err
                }else {
                    curr.add_deet(lt.trim(),rt.trim());
                }
                
            }else {
                if fnd { 
                    res.items.push(curr);
                }
                //new curr 
                curr = Lz::new(a.trim());
                fnd = true;
            }

        }
        if !fnd {
            return Err(LzErr::NoItemsFound);

        }
        res.items.push(curr);
        Ok(res)
    }

    pub fn load<P:AsRef<Path>>(fname:P)->Result<LzList,LzErr>{
        let mut f = File::open(fname)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        LzList::read(&s)
    }

    pub fn load_all<'a, IT, ST:AsRef<Path>+Debug>(fnames:&mut IT)->LzList
        where IT:Iterator<Item = ST>
    {
        let mut jn = Vec::new();
        for n in fnames{ 
            match LzList::load(&n){
                Ok(lz)=>jn.push(lz),
                _=>{},
            }
        }
        LzList::join(&jn[..])
    }

    pub fn empty()->LzList{
        LzList{
            items:Vec::new(),
        }
    }
    pub fn len(&self )->usize{
        self.items.len()
    }

    pub fn join(ll:&[LzList])->LzList{
        let mut ritems:Vec<Lz> = Vec::new();
        for l in ll.iter() {
            for i in l.iter(){
                let p = (*i).clone();
                ritems.push(p);
            }
        }
        LzList{
            items:ritems,
        }
    }


    pub fn iter(&self)->::std::slice::Iter<Lz>{
        self.items.iter()
    }

    pub fn lz_by_name(&self,s:&str)->Option<Lz>{
        for i in self.items.iter() {
            if i.name == s {
                return Some(i.clone());
            }
        }
        None
    }
}

impl <'a> SGetter<&'a str> for LzList {
    fn get_s(&self,s:&str)->Option<String>{
        let (lt,rt) = brace::split_on(s,'.');
        if rt != "" {
            for ref lz in &self.items{
                if lt == lz.name {
                    return lz.get_s(rt);
                }
            }
            return None; 
        }

        if self.items.len() == 0 {
            return None;
        }
        self.items[0].get_s(s)
    }
}

impl IntoIterator for LzList {
    type Item = Lz;
    type IntoIter = ::std::vec::IntoIter<Lz>;
   
    fn into_iter(self)->::std::vec::IntoIter<Lz>{
        self.items.into_iter()
    }
}

impl<'a> IntoIterator for &'a LzList {
    type Item = &'a Lz;
    type IntoIter = ::std::slice::Iter<'a,Lz>;
    fn into_iter(self)->::std::slice::Iter<'a,Lz>{
        (&self.items).into_iter()
    }
}


#[cfg(test)]
mod tests {
    use lzlist::LzList;
    use get::SGetter;
    #[test]
    fn tload_all(){
        let ll = LzList::load_all(&mut ["test_data/lztest.lz","test_data/lztest2.lz"].iter());
        assert_eq!(ll.len(),3);

        assert_eq!(ll.get_t_def("cf3.poop",0),7);
        
        //assert_eq!(l2.len(),3);
        
    }
}




