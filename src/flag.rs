
use std::env;
use std::str::FromStr;

pub fn ss_a(s:&str, iter:&mut Iterator<Item=String>)->Option<String>{
    let mut fnd = false;
    for a in iter{
        if fnd {
            return Some(a.clone());
        }
        if a == s {
            fnd = true;
        }
        println!("{}",a);
    }

    if fnd { return Some(String::from(""));}

    None
}


pub fn ss(s:&str)->Option<String>{
    ss_a(s,&mut env::args())
}

pub fn ss_def(s:&str,def:&str)->String{
    match ss(s) {
        Some(r)=> r,
        None=> String::from(def.clone()),
    }
}

pub fn t<T:FromStr>(s:&str)->Result<T,String>{
    match ss(s){
        Some(r)=>{
            match r.parse::<T>() {
                Result::Ok(n)=> Result::Ok(n),
                Result::Err(_)=> Result::Err("no parse".to_string()),
            }
        }
        None=>Result::Err("No find".to_string()),
    }
}

pub fn t_def<T:FromStr>(s:&str,def:T)->T{
    match t::<T>(s) {
        Result::Ok(r)=>r,
        Result::Err(_)=>def,
    }
}
