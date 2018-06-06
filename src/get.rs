use std::str::FromStr;


pub trait SGetter {
    fn get_s(&self,s:&str)->Option<String>;
    
    fn get_s_def(&self,s:&str,def:&str)->String{
        match self.get_s(s) {
            Some(r)=>r,
            None=>def.to_string()
        }
    }

    fn get_t<T:FromStr>(&self,s:&str)->Option<T>{
        match self.get_s(s) {
            Some(r)=>{
                match r.parse::<T>(){
                    Ok(res)=>Some(res),
                    Err(_)=>None,
                }
            }
            None=>None
        }
    }

    fn get_t_def<T:FromStr>(&self,s:&str,def:T)->T{
        match self.get_t::<T>(s) {
            Some(res)=>res,
            None=>def,
        }
    }
    
}
