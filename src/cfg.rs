use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

struct lz{
    name:String,
    deets:HashMap<String,String>,
}

pub struct Cfg {
     items:Vec<lz>,
}

impl Cfg {
    pub fn read(s :&String)->Result<Cfg,String>{
        let sp = s.split("\n");
        let mut res = Cfg{
            items:Vec::new(),
        };
        for (k,a) in sp.enumerate() {
            if a == "" {continue;}
            res.items.push(lz{name:a.to_string(),deets:HashMap::new()}); 
            print!("{} : {}\n",k,a);
        }
        Ok(res)
    }

    pub fn load(fname:&str)->Result<Cfg,String>{
        let mut fok = File::open(fname);
        let mut s = String::new();
        match fok {
            Ok(mut f)=>{
                f.read_to_string(&mut s);
                Cfg::read(&s)
            },
            Err(_)=> Err("Could not read file".to_string()),
            
        }
    }
    pub fn len(&self )->usize{
        self.items.len()
    }
}


