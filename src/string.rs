//string utilities for

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

pub fn replace(s:&str,f:&Fn(&str)->&str)->String{
    let mut res = "".to_string();
    for c in s.chars(){
        if c == 'a'{
            res.push_str(f("a"));
            continue;
        }
        res.push( c);
    }
    return res;
}
