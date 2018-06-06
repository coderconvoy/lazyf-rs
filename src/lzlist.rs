use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use brace;

//use std::io::prelude::*;
use std::collections::HashMap;

pub struct Lz{
    pub name:String,
    deets:HashMap<String,String>,
}

impl Lz{
    pub fn new(line:String)->Lz{
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
            res.deets.insert(format!("ext{}",k-1),v.trim().to_string());
        }
        res
    }
    
    pub fn get(&self,s : &str)->Option<String>{
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
    pub fn read(s :&String)->Result<LzList,String>{
        let sp = s.split("\n");
        let mut res = LzList{
            items:Vec::new(),
        };
        let mut fnd = false;

        let mut curr = Lz::new("".to_string());

        for a in sp {//TODO errors with line nums
            if a == "" {continue;}
            let tabbed = match a.chars().nth(0) {
                Some(c)=>c.is_whitespace(),
                _=> true,
            };
            let a = a.trim_left();
            if &a[..1] == "#" {continue;}

            if tabbed {
                //new property
                let (lt,rt) = brace::split_on(a,':');
                if rt == "" {
                    //TODO Err
                }else {
                    curr.deets.insert(lt.trim().to_string(),rt.trim().to_string());
                }
                
            }else {
                if fnd { 
                    res.items.push(curr);
                }
                //new curr 
                curr = Lz::new(a.trim().to_string());
                fnd = true;
            }

        }
        if !fnd {
            return Err("No Items found".to_string())

        }
        res.items.push(curr);
        Ok(res)
    }

    pub fn load(fname:&str)->Result<LzList,String>{
        let fok = File::open(fname);
        let mut s = String::new();
        match fok {
            Ok(mut f)=>{
                match f.read_to_string(&mut s) {
                    Ok(_)=>LzList::read(&s),
                    Err(e)=>Err(format!("read_to_string failed:{}",e)),
                }
            },
            Err(_)=> Err("Could not read file".to_string()),
            
        }
    }

    pub fn empty()->LzList{
        LzList{
            items:Vec::new(),
        }
    }
    pub fn len(&self )->usize{
        self.items.len()
    }

    pub fn get(&self,s:&str)->Option<String>{
        let (lt,rt) = brace::split_on(s,'.');
        if rt != "" {
            for ref lz in &self.items{
                if lt == lz.name {
                    return lz.get(rt);
                }
            }
            return None; 
        }

        if self.items.len() == 0 {
            return None;
        }
        self.items[0].get(s)
    }

    pub fn get_def(&self,s:&str,def:&str)->String{
        match self.get(s) {
            Some(r)=>r,
            None=>def.to_string()
        }
    }

    pub fn get_t<T:FromStr>(&self,s:&str)->Option<T>{
        match self.get(s) {
            Some(r)=>{
                match r.parse::<T>(){
                    Ok(res)=>Some(res),
                    Err(_)=>None,
                }
            }
            None=>None
        }
    }

    pub fn get_t_def<T:FromStr>(&self,s:&str,def:T)->T{
        match self.get_t::<T>(s) {
            Some(res)=>res,
            None=>def,
        }
    }

    pub fn iter(&self)->::std::slice::Iter<Lz>{
        self.items.iter()
    }
}

impl IntoIterator for LzList {
    type Item = Lz;
    type IntoIter = ::std::vec::IntoIter<Lz>;
   
    fn into_iter(self)->::std::vec::IntoIter<Lz>{
        self.items.into_iter()
    }
}





