
use flag;
use lzlist;
use string;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn args(){
    let v = ["-a","60"];
    let r = flag::ss_a("-a",&mut v.into_iter().map(|&a|String::from(a)));
    assert_eq!(r,Result::Ok("60".to_string()));
}

#[test]
fn reader(){
    let f = lzlist::LzList::load("test_data/lztest.lz");

    match f {
        Ok(c)=>{
            assert_eq!( c.len(),2); 
            assert_eq!( c.get("config.ext0"),Some("4".to_string()));
            assert_eq!( c.get_t_def("config.ext0",0),4);
            assert_eq!(c.get("c2.lesson"),Some("3".to_string()));      

            for (i,v) in c.iter().enumerate(){
                if i == 0 {
                    assert_eq!(v.name , "config")
                }
            }
            assert_eq!(c.get("c3.poo"),None);
        },
        Err(e)=>assert!(false,"Error {}",e),
    }

    //assert_eq!(r,Result::Err("poo".to_string()));
}

fn strrep1(s:&str)->&str{
    "plop"
}

#[test]
fn string_edit() {
    assert_eq!(string::replace("cdab",&|s|{
        "poop"
    }),"cdpoopb");
    assert_eq!(string::replace("ppab",&strrep1),"ppplopb");
}
