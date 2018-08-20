//! # Get - SGetter
//!
//! This module provides a mechanism for asking for anything from the lazyf format.
//! As long as it can be parsed from a string
//! -- That is anything that implements the FromStr trait.
//!
//! To implement it provide the get_s method returning a String

use std::str::FromStr;



pub trait SGetter<IT> {
    //IT = IndexType
    //RT = ResultType
    fn get_s(&self,s:IT)->Option<String>;
    
    fn get_s_def(&self,s:IT,def:&str)->String{
        self.get_s(s).unwrap_or(def.to_string()) 
    }

    fn get_t<RT:FromStr>(&self,s:IT)->Result<RT,GetErr>{
        match self.get_s(s) {
            Some(r)=>{
                match r.parse::<RT>(){
                    Ok(res)=>Ok(res),
                    Err(_)=>Err(GetErr::NoParse(r.to_string())),
                }
            }
            None=>Err(GetErr::NotFound),
        }
    }

    fn get_t_def<RT:FromStr>(&self,s:IT,def:RT)->RT{
        self.get_t(s).unwrap_or(def)
    }
    
}
