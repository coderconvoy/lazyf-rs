
use std::env;
use get::SGetter;

pub fn ss_get(s:&str, iter:&mut Iterator<Item=String>)->Option<String>{
    let mut fnd = false;
    for a in iter{
        if fnd {
            return Some(a.clone());
        }
        if a == s {
            fnd = true;
        }
    }

    if fnd { return Some(String::from(""));}

    None
}

pub struct Fg{}
    
pub struct FgTest{
    v:Vec<String>,
}

impl <'a> SGetter<&'a str> for Fg{
    fn get_s(&self,s:&str)->Option<String>{
        return ss_get(&s,&mut env::args())
    }
}

impl <'a> SGetter<&'a str> for FgTest{
    fn get_s(&self,s:&str)->Option<String>{
        ss_get(s,&mut self.v.clone().into_iter())
    }
}


impl FgTest{
    pub fn new(v:Vec<String>)->FgTest{
        FgTest{
            v:v,
        }
    }
}


