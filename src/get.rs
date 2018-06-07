use std::str::FromStr;


pub trait SGetter<IT> {
    //IT = IndexType
    //RT = ResultType
    fn get_s(&self,s:IT)->Option<String>;
    
    fn get_s_def(&self,s:IT,def:&str)->String{
        match self.get_s(s) {
            Some(r)=>r,
            None=>def.to_string()
        }
    }

    fn get_t<RT:FromStr>(&self,s:IT)->Option<RT>{
        match self.get_s(s) {
            Some(r)=>{
                match r.parse::<RT>(){
                    Ok(res)=>Some(res),
                    Err(_)=>None,
                }
            }
            None=>None
        }
    }

    fn get_t_def<RT:FromStr>(&self,s:IT,def:RT)->RT{
        match self.get_t(s) {
            Some(res)=>res,
            None=>def,
        }
    }
    
}
