//brace takes a string with {} as deliminators
//and can replace the contents with something else

use std::env;

//A simple Util function for this library
pub fn split_on(s:&str,c:char)->(&str,&str){
        match s.find(c){
            Some(n)=>{
                let (lt,rt) = s.split_at(n);
                let rt = &rt[1..];
                (lt,rt)
            },
            None=>(s,"")
        }
}

pub fn replace(s:&str,f:&Fn(&str)->String)->String{
    let mut res = "".to_string();

    let mut depth = 0;
    let mut midepth = 0; //max inner depth
    let mut esc = false;
    let mut temp = "".to_string();
    for c in s.chars(){
        if esc {
            res.push(c) ;
            esc = false;
            continue;
        }
        if c == '\\'{
            esc = true; 
            continue;
        }
        if c == '{' {
            if depth == 0 {
                temp = "".to_string();
            }else{
                temp.push('{');
            }
            depth +=1;
            if midepth < depth {midepth +=1}
            continue;
        }
        if depth == 0 {
            res.push(c);
            continue;
        }
        if c == '}' {
            depth -=1;
            if depth == 0{
                if midepth >1 {
                    temp = replace(&temp,f);
                } 
                res.push_str(&f(&temp));
                
                continue;
            }
        }
        temp.push(c);
    }
    if depth > 1 {
        temp = replace(&temp,f);
    }
    if depth > 0 {
        res.push_str(&f(&temp));
    }
    return res;
}


pub fn env(s:&str)->String{
    match env::var_os(s) {
        Some(ros)=> match ros.to_str(){
            Some(res)=>String::from(res),
            None=>String::from(""),
        },
        None => String::from(""),
    }
}

pub fn env_replace(s:&str)->String{
    replace(s,&env)
}


